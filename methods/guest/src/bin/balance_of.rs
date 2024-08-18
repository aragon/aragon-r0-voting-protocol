#![allow(unused_doc_comments)]
#![no_main]

use alloy_primitives::{Address, U256};
use alloy_sol_types::{sol, SolValue};
use risc0_steel::{
    config::ETH_SEPOLIA_CHAIN_SPEC, ethereum::EthEvmInput, Contract, EvmBlockHeader, SolCommitment,
};
use risc0_zkvm::guest::env;
use serde::{Deserialize, Serialize};

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

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Restaking {
    address: Address,
    voting_power_strategy: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct Asset {
    token: Address,
    chain_id: u64,
    voting_power_strategy: String,
    delegation_strategy: String,
    restaking: Vec<Restaking>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
struct RiscVotingProtocolConfig {
    voting_protocol_version: String,
    assets: Vec<Asset>,
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
    let env = input.into_env().with_chain_spec(&ETH_SEPOLIA_CHAIN_SPEC);

    let config_call = ConfigContract::getConfigCall {};
    let config_returns = Contract::new(config_contract, &env)
        .call_builder(&config_call)
        .call();
    println!("Config Returns: {:?}", config_returns._0);

    let config = serde_json::from_str::<RiscVotingProtocolConfig>(&config_returns._0).unwrap();

    // Get the total voting power of the account across all assets.
    let total_voting_power = config
        .assets
        .iter()
        .map(|asset| {
            assert_eq!(asset.chain_id, env.header().number());
            let asset_contract = Contract::new(asset.token, &env);
            let balance_call = IERC20::balanceOfCall { account };
            let balance = asset_contract.call_builder(&balance_call).call();
            U256::from(balance._0)
        })
        .sum::<U256>();

    println!("Total voting power: {}", total_voting_power);

    assert!(total_voting_power >= U256::from(1));

    // Commit the block hash and number used when deriving `view_call_env` to the journal.
    let journal = Journal {
        commitment: env.block_commitment(),
        config_contract,
    };
    env::commit_slice(&journal.abi_encode());
}
