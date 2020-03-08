use crate::{Action, MatchConfig, Player, Strategy};
use rand::Rng;

pub struct StrategyD {}

impl StrategyD {
  pub fn new() -> StrategyD {
    StrategyD {}
  }
}

impl Strategy for StrategyD {
  fn name(&self) -> &str {
    "D"
  }
  fn description(&self) -> &str {
    "Round 1: Cooperate\n\
    Round 2 onward, copy what my opponent did in the previous round.\n\
    Whenever I defect, increase X by 1. Whenever I would defect, do a probability check with a 0.02*X probability of choosing to cooperate instead.\n\
    If I choose to cooperate instead of defect, reduce X to 0."
  }
  fn create_player<'a>(&self, _config: &'a MatchConfig) -> Box<dyn Player<'a> + 'a> {
    Box::new(PlayerD {
      x: 0,
      rng: rand::thread_rng(),
    })
  }
  fn is_mixed(&self) -> bool {
    true
  }
}

struct PlayerD<R: Rng> {
  x: u32,
  rng: R,
}

impl<'a, R: Rng> Player<'a> for PlayerD<R> {
  fn first_round(&self) -> Action {
    Action::Cooperate
  }
  fn next_round(&mut self, opponent_previous: &Action) -> Action {
    match opponent_previous {
      Action::Cooperate => Action::Cooperate,
      Action::Defect => {
        let rand_0_to_1: f64 = self.rng.gen();
        let probability = 0.02 * self.x as f64;
        if rand_0_to_1 <= probability {
          self.x = 0;
          Action::Cooperate
        } else {
          self.x += 1;
          Action::Defect
        }
      }
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_always_cooperate_back() {
    let config = MatchConfig {
      num_rounds: 200,
      both_coop_points: 4,
      defect_against_coop_points: 7,
      coop_against_defect_points: 0,
      both_defect_points: 1,
    };
    let mut p = StrategyD::new().create_player(&config);
    assert_eq!(p.first_round(), Action::Cooperate);
    for _i in 0..10 {
      assert_eq!(p.next_round(&Action::Cooperate), Action::Cooperate);
    }
  }
}
