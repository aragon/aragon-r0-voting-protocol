use crate::{Asset, HostEvmEnv};
use alloy_primitives::{Address, Bytes, U256};
use anyhow::Result;
use risc0_steel::host::provider::Provider;
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

pub trait DelegationStrategy<P, H>
where
    P: Provider,
    H: risc0_steel::EvmBlockHeader,
{
    fn process(
        &self,
        env: &mut HostEvmEnv<P, H>,
        account: Address,
        asset: &Asset,
        additional_data: Bytes,
    ) -> Result<Vec<Delegation>>;
}

mod split_delegation;

pub use split_delegation::SplitDelegation;
