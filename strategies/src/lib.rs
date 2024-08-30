pub mod execution_strategies;
pub mod voting_strategies;

use alloy_primitives::{Address, U256};
use execution_strategies::*;
use risc0_steel::{EvmEnv, SolCommitment};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use voting_strategies::*;

pub struct Context {
    protocol_strategies: HashMap<String, Box<dyn ProtocolStrategy>>,
    execution_strategies: HashMap<String, Box<dyn ProtocolExecutionStrategy>>,
    env: EvmEnv<risc0_steel::StateDb, risc0_steel::ethereum::EthBlockHeader>,
}

impl Context {
    pub fn new(env: EvmEnv<risc0_steel::StateDb, risc0_steel::ethereum::EthBlockHeader>) -> Self {
        Self {
            protocol_strategies: HashMap::new(),
            execution_strategies: HashMap::new(),
            env,
        }
    }

    pub fn default(
        env: EvmEnv<risc0_steel::StateDb, risc0_steel::ethereum::EthBlockHeader>,
    ) -> Self {
        let mut protocol_strategies: HashMap<String, Box<dyn ProtocolStrategy>> = HashMap::new();
        protocol_strategies.insert("BalanceOf".to_string(), Box::new(BalanceOf));
        protocol_strategies.insert("GetPastVotes".to_string(), Box::new(GetPastVotes));

        let mut execution_strategies: HashMap<String, Box<dyn ProtocolExecutionStrategy>> =
            HashMap::new();
        execution_strategies.insert("MajorityVoting".to_string(), Box::new(MajorityVoting));

        Self {
            protocol_strategies,
            execution_strategies,
            env,
        }
    }

    pub fn add_strategy(&mut self, name: String, protocol_strategy: Box<dyn ProtocolStrategy>) {
        self.protocol_strategies.insert(name, protocol_strategy);
    }

    pub fn process_voting_strategy(&self, name: String, account: Address, asset: &Asset) -> U256 {
        if let Some(protocol_strategy) = self.protocol_strategies.get(&name) {
            protocol_strategy.process(&self.env, account, asset)
        } else {
            panic!("Strategy not found: {}", name);
        }
    }

    pub fn process_total_supply(&self, name: String, asset: &Asset) -> U256 {
        if let Some(protocol_strategy) = self.protocol_strategies.get(&name) {
            protocol_strategy.get_supply(&self.env, asset)
        } else {
            panic!("Strategy not found: {}", name);
        }
    }

    pub fn process_execution_strategy(
        &self,
        name: String,
        total_supply: U256,
        tally: [U256; 3],
    ) -> bool {
        if let Some(execution_strategy) = self.execution_strategies.get(&name) {
            execution_strategy.proof_execution(&self.env, total_supply, tally)
        } else {
            panic!("Strategy not found: {}", name);
        }
    }

    pub fn block_commitment(&self) -> SolCommitment {
        self.env.block_commitment()
    }
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Restaking {
    pub address: Address,
    pub voting_power_strategy: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Asset {
    pub token: Address,
    pub chain_id: u64,
    pub voting_power_strategy: String,
    pub delegation_strategy: String,
    pub restaking: Vec<Restaking>,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RiscVotingProtocolConfig {
    pub voting_protocol_version: String,
    pub assets: Vec<Asset>,
    pub execution_strategy: String,
}
