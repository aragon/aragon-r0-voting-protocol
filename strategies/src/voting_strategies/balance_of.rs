use super::ProtocolStrategy;

pub struct BalanceOf;
impl ProtocolStrategy for BalanceOf {
    fn process(&self, left: u64, right: u64) -> u64 {
        left + right
    }
}
