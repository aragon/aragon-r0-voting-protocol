use crate::{Asset, EthHostEvmEnv};
use alloy::{network::Network, providers::Provider, transports::Transport};
use alloy_primitives::{Address, U256};

pub trait VotingPowerStrategy<T, N, P, H>
where
    T: Transport + Clone,
    N: Network,
    P: Provider<T, N>,
    H: risc0_steel::EvmBlockHeader,
{
    fn process(&self, env: &mut EthHostEvmEnv<T, N, P, H>, account: Address, asset: &Asset)
        -> U256;
}

mod balance_of;
mod get_past_votes;
pub use balance_of::BalanceOf;
pub use get_past_votes::GetPastVotes;
