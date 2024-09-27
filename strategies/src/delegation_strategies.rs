use crate::Asset;
use alloy_primitives::{Address, U256};
use anyhow::Result;
use risc0_steel::EvmEnv;
use std::iter::FromIterator;

pub struct Delegation {
    pub delegate: Address,
    pub ratio: U256,
}

impl FromIterator<(Address, U256)> for Delegation {
    fn from_iter<I: IntoIterator<Item = (Address, U256)>>(iter: I) -> Self {
        let mut iter = iter.into_iter();

        // Take the first tuple from the iterator to create the struct
        if let Some((delegate, ratio)) = iter.next() {
            Delegation { delegate, ratio }
        } else {
            panic!("Iterator is empty, cannot create MyStruct");
        }
    }
}

pub trait DelegationStrategy {
    fn process(
        &self,
        env: &EvmEnv<risc0_steel::StateDb, risc0_steel::ethereum::EthBlockHeader>,
        account: Address,
        asset: &Asset,
        additional_data: Vec<u8>,
    ) -> Result<Vec<Delegation>>;
}

mod split_delegation;

pub use split_delegation::SplitDelegation;
