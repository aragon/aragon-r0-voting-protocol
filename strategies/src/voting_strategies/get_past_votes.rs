use super::ProtocolStrategy;
use alloy_primitives::U256;
use risc0_steel::EvmEnv;

pub struct GetPastVotes;
impl ProtocolStrategy for GetPastVotes {
    fn process(
        &self,
        env: &EvmEnv<risc0_steel::StateDb, risc0_steel::ethereum::EthBlockHeader>,
        left: u64,
        right: u64,
    ) -> U256 {
        U256::from(left + right)
    }
}
