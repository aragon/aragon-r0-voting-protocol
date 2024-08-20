use alloy_primitives::U256;
use risc0_steel::{EvmEnv, SolCommitment};
use std::collections::HashMap;
pub mod voting_strategies;
use voting_strategies::*;

pub struct Context {
    protocol_strategies: HashMap<String, Box<dyn ProtocolStrategy>>,
    env: EvmEnv<risc0_steel::StateDb, risc0_steel::ethereum::EthBlockHeader>,
}

impl Context {
    pub fn new(env: EvmEnv<risc0_steel::StateDb, risc0_steel::ethereum::EthBlockHeader>) -> Self {
        Self {
            protocol_strategies: HashMap::new(),
            env,
        }
    }

    pub fn default(
        env: EvmEnv<risc0_steel::StateDb, risc0_steel::ethereum::EthBlockHeader>,
    ) -> Self {
        let mut protocol_strategies: HashMap<String, Box<dyn ProtocolStrategy>> = HashMap::new();
        protocol_strategies.insert("BalanceOf".to_string(), Box::new(BalanceOf));
        protocol_strategies.insert("GetPastVotes".to_string(), Box::new(GetPastVotes));

        Self {
            protocol_strategies,
            env,
        }
    }

    pub fn add_strategy(&mut self, name: String, protocol_strategy: Box<dyn ProtocolStrategy>) {
        self.protocol_strategies.insert(name, protocol_strategy);
    }

    pub fn process_strategy(&self, name: String, left: u64, right: u64) -> U256 {
        if let Some(protocol_strategy) = self.protocol_strategies.get(&name) {
            protocol_strategy.process(&self.env, left, right)
        } else {
            panic!("Strategy not found: {}", name);
        }
    }

    pub fn block_commitment(&self) -> SolCommitment {
        self.env.block_commitment()
    }
}
