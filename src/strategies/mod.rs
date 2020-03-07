mod strategy_a;

pub enum Action {
  Cooperate,
  Defect,
}

pub trait Strategy {
  fn name(&self) -> &str;
  fn description(&self) -> &str;
  fn create_player(&self) -> Box<dyn Player>;
}

pub trait Player {
  fn first_round(&self) -> Action;
  fn next_round(&mut self, opponent_previous: &Action) -> Action;
}

pub fn get_strategies() -> Vec<Box<dyn Strategy>> {
  vec![Box::new(strategy_a::StrategyA::new())]
}
