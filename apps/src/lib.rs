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

pub mod voting_power_strategies;
use anyhow::Result;
use ethers::prelude::*;
use risc0_steel::{host::db::ProofDb, EvmBlockHeader, EvmEnv};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use voting_power_strategies::*;

pub(crate) type HostEvmEnv<P, H> = EvmEnv<ProofDb<P>, H>;

pub struct HostContext<'a, P, H> {
    voting_power_strategies: HashMap<String, Box<dyn VotingPowerStrategy<P, H>>>,
    env: &'a mut HostEvmEnv<P, H>,
}

impl<'a, P, H> HostContext<'a, P, H>
where
    P: risc0_steel::host::provider::Provider,
    H: EvmBlockHeader,
{
    pub fn default(env: &'a mut HostEvmEnv<P, H>) -> Self {
        let mut voting_power_strategies: HashMap<String, Box<dyn VotingPowerStrategy<P, H>>> =
            HashMap::new();
        voting_power_strategies.insert("BalanceOf".to_string(), Box::new(BalanceOf));
        // voting_power_strategies.insert("GetPastVotes".to_string(), Box::new(GetPastVotes));

        Self {
            voting_power_strategies,
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
}

// The input of the config
#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Restaking {
    pub address: alloy_primitives::Address,
    pub voting_power_strategy: String,
}

#[derive(Serialize, Deserialize)]
#[serde(rename_all = "camelCase")]
pub struct Asset {
    pub token: alloy_primitives::Address,
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

/// Wrapper of a `SignerMiddleware` client to send transactions to the given
/// contract's `Address`.
pub struct TxSender {
    chain_id: u64,
    client: SignerMiddleware<Provider<Http>, Wallet<k256::ecdsa::SigningKey>>,
    contract: Address,
}

impl TxSender {
    /// Creates a new `TxSender`.
    pub fn new(chain_id: u64, rpc_url: &str, private_key: &str, contract: &str) -> Result<Self> {
        let provider = Provider::<Http>::try_from(rpc_url)?;
        let wallet: LocalWallet = private_key.parse::<LocalWallet>()?.with_chain_id(chain_id);
        let client = SignerMiddleware::new(provider.clone(), wallet.clone());
        let contract = contract.parse::<Address>()?;

        Ok(TxSender {
            chain_id,
            client,
            contract,
        })
    }

    /// Send a transaction with the given calldata.
    pub async fn send(&self, calldata: Vec<u8>) -> Result<Option<TransactionReceipt>> {
        let tx = TransactionRequest::new()
            .chain_id(self.chain_id)
            .to(self.contract)
            .from(self.client.address())
            .data(calldata);

        log::info!("Transaction request: {:?}", &tx);

        let tx = self.client.send_transaction(tx, None).await?.await?;

        log::info!("Transaction receipt: {:?}", &tx);

        Ok(tx)
    }
}
