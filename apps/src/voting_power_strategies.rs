use crate::{Asset, HostEvmEnv};
use alloy_primitives::{Address, U256};
use risc0_steel::host::provider::Provider;

pub trait VotingPowerStrategy<P, H>
where
    P: Provider,
    H: risc0_steel::EvmBlockHeader,
{
    fn process(&self, env: &mut HostEvmEnv<P, H>, account: Address, asset: &Asset) -> U256;
}

mod balance_of;
pub use balance_of::BalanceOf;
