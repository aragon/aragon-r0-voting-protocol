use std::collections::HashMap;
pub mod voting_strategies;
use voting_strategies::*;

pub struct Context {
    protocol_strategies: HashMap<String, Box<dyn ProtocolStrategy>>,
}

impl Context {
    pub fn new() -> Self {
        Self {
            protocol_strategies: HashMap::new(),
        }
    }

    pub fn add_strategy(&mut self, name: String, protocol_strategy: Box<dyn ProtocolStrategy>) {
        self.protocol_strategies.insert(name, protocol_strategy);
    }

    pub fn process_strategy(&self, name: String, left: u64, right: u64) -> u64 {
        if let Some(protocol_strategy) = self.protocol_strategies.get(&name) {
            protocol_strategy.process(left, right)
        } else {
            panic!("Strategy not found: {}", name);
        }
    }
}

impl Default for Context {
    fn default() -> Self {
        let mut protocol_strategies: HashMap<String, Box<dyn ProtocolStrategy>> = HashMap::new();
        protocol_strategies.insert("BalanceOf".to_string(), Box::new(BalanceOf));
        protocol_strategies.insert("GetPastVotes".to_string(), Box::new(GetPastVotes));

        Self {
            protocol_strategies,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let context = Context::default();
        let result = context.process_strategy("BalanceOf".to_string(), 2, 2);
        assert_eq!(result, 4);
    }
}
