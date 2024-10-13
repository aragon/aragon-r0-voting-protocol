use crate::{Asset, EthHostEvmEnv};
use alloy::providers::Provider;
use alloy_primitives::{Address, Bytes, U256};
use anyhow::Result;
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
            println!("delegate: {}, ratio: {}", delegate, ratio);
            Delegation { delegate, ratio }
        } else {
            panic!("Iterator is empty, cannot create Delegation");
        }
    }
}

pub trait DelegationStrategy<P, H>
where
    P: Provider + revm::primitives::db::Database,
    H: risc0_steel::EvmBlockHeader,
{
    fn process(
        &self,
        env: &mut EthHostEvmEnv<P, H>,
        account: Address,
        asset: &Asset,
        additional_data: Bytes,
    ) -> Result<Vec<Delegation>>;
}

mod split_delegation;

pub use split_delegation::SplitDelegation;
