use crate::{Asset, EthHostEvmEnv};
use alloy::providers::Provider;
use alloy_primitives::{Address, U256};

pub trait VotingPowerStrategy<P, H>
where
    P: Provider,
    H: risc0_steel::EvmBlockHeader,
{
    fn process(&self, env: &mut EthHostEvmEnv<P, H>, account: Address, asset: &Asset) -> U256;
}

mod balance_of;
mod get_past_votes;
pub use balance_of::BalanceOf;
pub use get_past_votes::GetPastVotes;
