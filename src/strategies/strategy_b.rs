use crate::{Action, MatchConfig, Player, Strategy};

pub struct StrategyB {}

impl StrategyB {
  pub fn new() -> StrategyB {
    StrategyB {}
  }
}

impl Strategy for StrategyB {
  fn name(&self) -> &str {
    "B"
  }
  fn description(&self) -> &str {
    "if opponent has answered cooperate > 50% so far, answer defect; else cooperate"
  }
  fn create_player<'a>(&self, _config: &'a MatchConfig) -> Box<dyn Player + 'a> {
    Box::new(PlayerB {
      opponent_cooperate_plus_minus: 0,
    })
  }
  fn is_mixed(&self) -> bool {
    false
  }
}

struct PlayerB {
  opponent_cooperate_plus_minus: i32,
}

impl Player for PlayerB {
  fn first_round(&self) -> Action {
    Action::Cooperate
  }
  fn next_round(&mut self, opponent_previous: &Action) -> Action {
    // Update the current count.
    let delta = match opponent_previous {
      Action::Cooperate => 1,
      Action::Defect => -1,
    };
    self.opponent_cooperate_plus_minus += delta;
    if self.opponent_cooperate_plus_minus > 0 {
      Action::Defect
    } else {
      Action::Cooperate
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_player() {
    let config = MatchConfig {
      num_rounds: 200,
      both_coop_points: 4,
      defect_against_coop_points: 7,
      coop_against_defect_points: 0,
      both_defect_points: 1,
    };
    let mut p = StrategyB::new().create_player(&config);
    assert_eq!(p.first_round(), Action::Cooperate);
    // Round 1: (cooperate, defect). Player should cooperate next.
    assert_eq!(p.next_round(&Action::Defect), Action::Cooperate);
    // Round 2: (cooperate, cooperate).
    assert_eq!(p.next_round(&Action::Cooperate), Action::Cooperate);
    // Round 3: (cooperate, cooperate).
    assert_eq!(p.next_round(&Action::Cooperate), Action::Defect);
  }
}
