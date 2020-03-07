mod s_a;
use std::fmt;

pub enum Action {
  Cooperate,
  Defect,
}

pub trait Strategy {
  fn first_round(&self) -> Action;
  fn name(&self) -> &str;
  fn description(&self) -> &str;
  fn reset(&self) -> Box<dyn Strategy>;
}

impl fmt::Display for dyn Strategy {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    write!(f, "{}", self.name())
  }
}

pub fn get_strategies() -> Vec<Box<dyn Strategy>> {
  vec![Box::new(s_a::StrategyA::new())]
}
