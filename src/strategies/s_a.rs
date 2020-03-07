use super::Action;
use super::Strategy;

pub struct StrategyA {}

impl StrategyA {
  pub fn new() -> StrategyA {
    StrategyA {}
  }
}

impl Strategy for StrategyA {
  fn first_round(&self) -> Action {
    Action::Cooperate
  }
  fn name(&self) -> &str {
    "A"
  }
  fn description(&self) -> &str {
    "If I am losing, defect. If I am winning or tied, cooperate."
  }
  fn reset(&self) -> Box<dyn Strategy> {
    Box::new(StrategyA::new())
  }
}
