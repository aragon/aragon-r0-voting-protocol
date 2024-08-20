#![allow(unused_doc_comments)]
#![no_main]

use alloy_primitives::{Address, U256};
use alloy_sol_types::{sol, SolValue};
use risc0_steel::{config::ETH_SEPOLIA_CHAIN_SPEC, ethereum::EthEvmInput, Contract, SolCommitment};
use risc0_zkvm::guest::env;

risc0_zkvm::guest::entry!(main);

/// Specify the function to call using the [`sol!`] macro.
/// This parses the Solidity syntax to generate a struct that implements the `SolCall` trait.
sol! {
    /// ERC-20 balance function signature.
    interface IERC20 {
        function balanceOf(address account) external view returns (uint);
    }

    interface ConfigContract {
        function getConfig() external view returns (string memory);
    }
}

/// ABI encodable journal data.
sol! {
    struct Journal {
        SolCommitment commitment;
        address config_contract;
    }
}

fn main() {
    // Read the input from the guest environment.
    println!("Reading input from the guest environment");
    let input: EthEvmInput = env::read();
    let account: Address = env::read();
    let config_contract: Address = env::read();

    // Converts the input into a `EvmEnv` for execution. The `with_chain_spec` method is used
    // to specify the chain configuration. It checks that the state matches the state root in the
    // header provided in the input.
    let destination_chain_id = &ETH_SEPOLIA_CHAIN_SPEC;
    let env = input.into_env().with_chain_spec(destination_chain_id);

    let config_call = ConfigContract::getConfigCall {};
    let config_returns = Contract::new(config_contract, &env)
        .call_builder(&config_call)
        .call();
    println!("Config Returns: {:?}", config_returns._0);

    let config =
        serde_json::from_str::<strategies::RiscVotingProtocolConfig>(&config_returns._0).unwrap();

    let strategies_context = strategies::Context::default(env);

    // Get the total voting power of the account across all assets.
    let total_voting_power = config
        .assets
        .iter()
        .map(|asset| {
            strategies_context.process_strategy(asset.voting_power_strategy.clone(), account, asset)
            // assert_eq!(asset.chain_id, destination_chain_id.chain_id());
        })
        .sum::<U256>();

    println!("Total voting power: {}", total_voting_power);

    assert!(total_voting_power >= U256::from(1));

    // Commit the block hash and number used when deriving `view_call_env` to the journal.
    let journal = Journal {
        commitment: strategies_context.block_commitment(),
        config_contract,
    };
    env::commit_slice(&journal.abi_encode());
}
