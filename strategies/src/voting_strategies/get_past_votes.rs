use super::ProtocolStrategy;

pub struct GetPastVotes;
impl ProtocolStrategy for GetPastVotes {
    fn process(&self, left: u64, right: u64) -> u64 {
        left + right
    }
}
