pub trait ProtocolStrategy {
    fn process(&self, left: u64, right: u64) -> u64;
}

mod balance_of;
mod get_past_votes;

pub use balance_of::BalanceOf;
pub use get_past_votes::GetPastVotes;
