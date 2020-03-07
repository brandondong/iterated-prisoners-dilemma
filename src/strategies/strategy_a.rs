use super::Action;
use super::Player;
use super::Strategy;

pub struct StrategyA {}

impl StrategyA {
  pub fn new() -> StrategyA {
    StrategyA {}
  }
}

impl Strategy for StrategyA {
  fn name(&self) -> &str {
    "A"
  }
  fn description(&self) -> &str {
    "If I am losing, defect. If I am winning or tied, cooperate."
  }
  fn create_player(&self) -> Box<dyn Player> {
    Box::new(PlayerA {})
  }
}

struct PlayerA {}

impl Player for PlayerA {
  fn first_round(&self) -> Action {
    Action::Cooperate
  }
  fn next_round(&mut self, opponent_previous: &Action) -> Action {
    Action::Cooperate
  }
}
