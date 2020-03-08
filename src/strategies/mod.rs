mod strategy_a;
mod strategy_b;
mod strategy_c;
mod strategy_d;

#[derive(Clone, Debug, PartialEq)]
pub enum Action {
  Cooperate,
  Defect,
}

pub trait Strategy {
  fn name(&self) -> &str;
  fn description(&self) -> &str;
  fn create_player(&self) -> Box<dyn Player>;
  fn is_mixed(&self) -> bool;
}

pub trait Player {
  fn first_round(&self) -> Action;
  fn next_round(&mut self, opponent_previous: &Action) -> Action;
}

pub fn get_strategies() -> Vec<Box<dyn Strategy>> {
  vec![
    Box::new(strategy_a::StrategyA::new()),
    Box::new(strategy_b::StrategyB::new()),
    Box::new(strategy_c::StrategyC::new()),
    Box::new(strategy_d::StrategyD::new()),
  ]
}
