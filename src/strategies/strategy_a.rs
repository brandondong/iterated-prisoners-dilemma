use super::super::evaluate_actions;
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
    Box::new(PlayerA {
      points_plus_minus: 0,
      previous_action: Action::Cooperate,
    })
  }
}

struct PlayerA {
  points_plus_minus: i32,
  previous_action: Action,
}

impl Player for PlayerA {
  fn first_round(&self) -> Action {
    Action::Cooperate
  }
  fn next_round(&mut self, opponent_previous: &Action) -> Action {
    // Update the current score.
    let (x1, x2) = evaluate_actions(&self.previous_action, opponent_previous);
    self.points_plus_minus += x1 as i32 - x2 as i32;

    let a = if self.points_plus_minus >= 0 {
      Action::Cooperate
    } else {
      Action::Defect
    };
    self.previous_action = a.clone();
    a
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_player() {
    let mut p = StrategyA::new().create_player();
    assert_eq!(p.first_round(), Action::Cooperate);
    // Round 1: (cooperate, defect). Player should defect next.
    assert_eq!(p.next_round(&Action::Defect), Action::Defect);
    // Round 2: (defect, defect).
    assert_eq!(p.next_round(&Action::Defect), Action::Defect);
    // Round 3: (defect, cooperate).
    assert_eq!(p.next_round(&Action::Cooperate), Action::Cooperate);
    // Round 4: (cooperate, cooperate).
    assert_eq!(p.next_round(&Action::Cooperate), Action::Cooperate);
  }
}
