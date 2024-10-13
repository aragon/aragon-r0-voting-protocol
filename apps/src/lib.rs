// Copyright 2024 RISC Zero, Inc.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

// The following library provides utility functions to help with sending
// transactions to a deployed app contract on Ethereum.

pub mod delegation_strategies;
pub mod voting_power_strategies;
use alloy::providers::Provider;
use alloy_primitives::Bytes;
use anyhow::Result;
use delegation_strategies::*;
use risc0_steel::{
    ethereum::EthEvmEnv,
    host::{db::ProofDb, HostCommit},
    EvmBlockHeader,
};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use voting_power_strategies::*;

type EthHostEvmEnv<D, C> = EthEvmEnv<ProofDb<D>, HostCommit<C>>;
/// Wrapper for the commit on the host.

pub struct HostContext<'a, P, H> {
    voting_power_strategies: HashMap<String, Box<dyn VotingPowerStrategy<P, H>>>,
    delegation_strategies: HashMap<String, Box<dyn DelegationStrategy<P, H>>>,
    env: &'a mut EthHostEvmEnv<P, H>,
}

impl<'a, P, H> HostContext<'a, P, H>
where
    P: Provider + revm::primitives::db::Database,
    H: EvmBlockHeader,
{
    pub fn default(env: &'a mut EthHostEvmEnv<P, H>) -> Self {
        let mut voting_power_strategies: HashMap<String, Box<dyn VotingPowerStrategy<P, H>>> =
            HashMap::new();
        voting_power_strategies.insert("BalanceOf".to_string(), Box::new(BalanceOf));
        voting_power_strategies.insert("GetPastVotes".to_string(), Box::new(GetPastVotes));

        let mut delegation_strategies: HashMap<String, Box<dyn DelegationStrategy<P, H>>> =
            HashMap::new();
        delegation_strategies.insert("SplitDelegation".to_string(), Box::new(SplitDelegation));

        Self {
            voting_power_strategies,
            delegation_strategies,
            env,
        }
    }

    pub fn add_strategy(
        &mut self,
        name: String,
        voting_power_strategy: Box<dyn VotingPowerStrategy<P, H>>,
    ) {
        self.voting_power_strategies
            .insert(name, voting_power_strategy);
    }

    pub fn process_voting_power_strategy(
        &mut self,
        name: String,
        account: alloy_primitives::Address,
        asset: &Asset,
    ) -> alloy_primitives::U256 {
        if let Some(voting_power_strategy) = self.voting_power_strategies.get(&name) {
            voting_power_strategy.process(&mut self.env, account, asset)
        } else {
            panic!("Strategy not found: {}", name);
        }
    }

    pub fn process_delegation_strategy(
        &mut self,
        account: alloy_primitives::Address,
        asset: &Asset,
        additional_data: Bytes,
    ) -> Result<Vec<Delegation>> {
        if let Some(delegation_strategy) = self
            .delegation_strategies
            .get(asset.delegation.strategy.as_str())
        {
            delegation_strategy.process(&mut self.env, account, asset, additional_data)
        } else {
            panic!("Strategy not found: {}", asset.delegation.strategy);
        }
    }
}

// The input of the config
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct DelegationObject {
    pub contract: alloy_primitives::Address,
    pub strategy: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Asset {
    pub contract: alloy_primitives::Address,
    pub chain_id: u64,
    pub voting_power_strategy: String,
    pub delegation: DelegationObject,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct RiscVotingProtocolConfig {
    pub voting_protocol_version: String,
    pub assets: Vec<Asset>,
    pub execution_strategy: String,
}
