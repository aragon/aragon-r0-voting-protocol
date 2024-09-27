use alloy_primitives::{Address, U256};
use alloy_sol_types::{sol, SolCall};
use anyhow::Result;
use apps::{HostContext, TxSender};
use aragon_zk_voting_protocol_methods::VOTING_PROTOCOL_ELF;
use clap::Parser;
use risc0_ethereum_contracts::groth16::encode;
use risc0_steel::{config::ETH_SEPOLIA_CHAIN_SPEC, ethereum::EthEvmEnv, Contract};
use risc0_zkvm::{default_prover, ExecutorEnv, ProverOpts, VerifierContext};
use tracing_subscriber::EnvFilter;

sol! {
    /// ERC-20 balance function signature.
    /// This must match the signature in the guest.
    interface IERC20 {
        function balanceOf(address account) external view returns (uint);
    }
    interface ConfigContract {
        function getVotingProtocolConfig() external view returns (string memory);
    }
}

sol!("../contracts/IMajorityVoting.sol");

/// Arguments of the publisher CLI.
#[derive(Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
    /// Ethereum chain ID
    #[clap(long)]
    chain_id: u64,

    /// Ethereum Node endpoint.
    #[clap(long, env)]
    eth_wallet_private_key: String,

    /// Ethereum Node endpoint.
    #[clap(long, env)]
    rpc_url: String,

    /// Ethereum block number.
    #[clap(long)]
    block_number: Option<u64>,

    /// Voter's signature
    #[clap(long)]
    voter_signature: String,

    /// Account address to read the balance_of on Ethereum
    #[clap(long)]
    voter: Address,

    /// Account address of the DAO the voter is voting for
    #[clap(long)]
    dao_address: Address,

    /// Proposal ID
    #[clap(long)]
    proposal_id: U256,

    /// Vote direction
    #[clap(long)]
    direction: u8,

    /// Voter's balance
    #[clap(long)]
    balance: U256,

    /// Counter's contract address on Ethereum
    #[clap(long)]
    config_contract: Address,

    /// ERC20 contract address on Ethereum
    #[clap(long)]
    token: Address,

    /// Additional delegation data
    #[clap(long)]
    additional_delegation_data: Vec<u8>,
}

fn to_hex_string(bytes: &[u8]) -> String {
    // Convert each byte to its hexadecimal representation and collect into a single String
    bytes.iter().map(|byte| format!("{:02x}", byte)).collect()
}

fn main() -> Result<()> {
    // Initialize tracing. In order to view logs, run `RUST_LOG=info cargo run`
    tracing_subscriber::fmt()
        .with_env_filter(EnvFilter::from_default_env())
        .init();

    // parse the command line arguments
    let args = Args::parse();

    // Create an EVM environment from an RPC endpoint and a block number. If no block number is
    // provided, the latest block is used.
    let mut env = EthEvmEnv::from_rpc(&args.rpc_url, args.block_number)?;
    //  The `with_chain_spec` method is used to specify the chain configuration.
    env = env.with_chain_spec(&ETH_SEPOLIA_CHAIN_SPEC);

    // Making the preflighs. This step is mandatory
    let config_call = ConfigContract::getVotingProtocolConfigCall {};
    let mut config_contract = Contract::preflight(args.config_contract, &mut env);
    let config_returns = config_contract.call_builder(&config_call).call()?;
    println!("Config string: {:?}", config_returns._0);

    let config =
        serde_json::from_str::<apps::RiscVotingProtocolConfig>(&config_returns._0).unwrap();
    let mut strategies_context = HostContext::default(&mut env);
    // Get the total voting power of the voter across all assets.
    let total_voting_power: U256 = config
        .assets
        .iter()
        .map(|asset| {
            // Get the accounts whost voting power is delegated to the voter.
            let delegations = strategies_context.process_delegation_strategy(
                asset.delegation_strategy.clone(),
                args.voter,
                asset,
                args.additional_delegation_data.clone(),
            );
            if delegations.is_err() {
                println!("Delegations given are not correct");
                assert!(false);
            }
            delegations
                .unwrap()
                .iter()
                .fold(U256::from(0), |acc, delegation| {
                    strategies_context.process_voting_power_strategy(
                        asset.voting_power_strategy.clone(),
                        delegation.delegate,
                        asset,
                    ) + acc
                })

            // assert_eq!(asset.chain_id, destination_chain_id.chain_id());
        })
        .sum::<U256>();

    println!("Total voting power: {}", total_voting_power);
    // Prepare the function call
    /*
        let call = IERC20::balanceOfCall {
            account: args.voter,
        };

        // Preflight the call to execute the function in the guest.
        let mut contract = Contract::preflight(args.token, &mut env);
        let returns = contract.call_builder(&call).call()?;
        println!(
            "For block {} calling `{}` on {} returns: {}",
            env.header().number(),
            IERC20::balanceOfCall::SIGNATURE,
            args.token,
            returns._0
        );
    */

    println!("proving...");

    let view_call_input = env.into_input()?;
    let env = ExecutorEnv::builder()
        .write(&view_call_input)?
        .write(&args.voter_signature)?
        .write(&args.voter)?
        .write(&args.dao_address)?
        .write(&args.proposal_id)?
        .write(&args.direction)?
        .write(&args.balance)?
        .write(&args.config_contract)?
        .build()?;

    let receipt = default_prover()
        .prove_with_ctx(
            env,
            &VerifierContext::default(),
            VOTING_PROTOCOL_ELF,
            &ProverOpts::groth16(),
        )?
        .receipt;
    println!("proving...done");

    // Create a new `TxSender`.
    let tx_sender = TxSender::new(
        args.chain_id,
        &args.rpc_url,
        &args.eth_wallet_private_key,
        &args.config_contract.to_string(),
    )?;

    // Encode the groth16 seal with the selector
    let seal = encode(receipt.inner.groth16()?.seal.clone())?;
    let journal_bytes = receipt.journal.bytes.as_slice();
    let seal_bytes = seal.as_slice();

    println!("journalData: {:?}", to_hex_string(journal_bytes));
    println!("seal: {:?}", to_hex_string(seal_bytes));

    // Encode the function call for `Plugin.vote(journal, seal)`.
    let calldata = IMajorityVoting::voteCall {
        journalData: receipt.journal.bytes.into(),
        seal: seal.into(),
    }
    .abi_encode();

    // Send the calldata to Ethereum.
    println!("sending tx...");

    let runtime = tokio::runtime::Runtime::new()?;
    runtime.block_on(tx_sender.send(calldata))?;
    println!("sending tx...done");

    Ok(())
}
