use super::Action;
use super::Player;
use super::Strategy;

pub struct StrategyC {}

impl StrategyC {
  pub fn new() -> StrategyC {
    StrategyC {}
  }
}

impl Strategy for StrategyC {
  fn name(&self) -> &str {
    "C"
  }
  fn description(&self) -> &str {
    "#1 Cooperate by default.\n\
    #2 If the opponent I'm playing has cooperated as well for two turn, start defecting. Continue defecting as long as the opponent is cooperating.\n\
    #3 If i'm defecting, and the opponent is defecting, resume #1 and cooperate until #2 criteria is met.\n\
    #4 if I'm cooperating and my opponent is defecting, defect for 2 rounds and resume #1"
  }
  fn create_player(&self) -> Box<dyn Player> {
    Box::new(PlayerC {
      state: State::CoopDefault,
      opponent_previous_previous: Action::Defect,
    })
  }
}

// Represent the strategy as a state machine.
enum State {
  CoopDefault,
  DefectUntilPunished,
  TwoRoundPunish,
}

struct PlayerC {
  state: State,
  opponent_previous_previous: Action,
}

impl Player for PlayerC {
  fn first_round(&self) -> Action {
    Action::Cooperate
  }
  fn next_round(&mut self, opponent_previous: &Action) -> Action {
    let action = match self.state {
      State::CoopDefault => {
        if let Action::Defect = opponent_previous {
          // Rule #4.
          self.state = State::TwoRoundPunish;
          Action::Defect
        } else if let Action::Cooperate = self.opponent_previous_previous {
          // Rule #2.
          self.state = State::DefectUntilPunished;
          Action::Defect
        } else {
          Action::Cooperate
        }
      }
      State::TwoRoundPunish => {
        // Already punished the previous round when transitioning to this state.
        self.state = State::CoopDefault;
        Action::Defect
      }
      State::DefectUntilPunished => {
        if let Action::Defect = opponent_previous {
          self.state = State::CoopDefault;
          Action::Cooperate
        } else {
          Action::Defect
        }
      }
    };
    self.opponent_previous_previous = opponent_previous.clone();
    action
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_two_round_punish() {
    let mut p = StrategyC::new().create_player();
    assert_eq!(p.first_round(), Action::Cooperate);
    // Round 1: (cooperate, defect). Punish defection for two rounds.
    assert_eq!(p.next_round(&Action::Defect), Action::Defect);
    // Round 2: (defect, defect).
    assert_eq!(p.next_round(&Action::Defect), Action::Defect);
    // Round 3: (cooperate, cooperate).
    assert_eq!(p.next_round(&Action::Cooperate), Action::Cooperate);
  }

  #[test]
  fn test_defect_until_punished() {
    let mut p = StrategyC::new().create_player();
    // Round 1: (cooperate, cooperate).
    assert_eq!(p.next_round(&Action::Cooperate), Action::Cooperate);
    // Round 2: (cooperate, cooperate).
    assert_eq!(p.next_round(&Action::Cooperate), Action::Defect);
    // Round 3: (defect, cooperate).
    assert_eq!(p.next_round(&Action::Cooperate), Action::Defect);
    // Round 3: (defect, defect).
    assert_eq!(p.next_round(&Action::Defect), Action::Cooperate);
  }
}
