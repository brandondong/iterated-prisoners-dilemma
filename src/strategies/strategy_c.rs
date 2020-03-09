use crate::{Action, MatchConfig, Player, Strategy};

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
  fn create_player<'a>(&self, _config: &'a MatchConfig) -> Box<dyn Player + 'a> {
    Box::new(PlayerC {
      state: State::DefaultState,
      previous_action: Action::Cooperate,
      opponent_previous_previous: Action::Defect,
    })
  }
  fn is_mixed(&self) -> bool {
    false
  }
}

// Represent the strategy as a state machine.
enum State {
  DefaultState,
  DefectUntilPunished,
  TwoRoundPunish,
}

struct PlayerC {
  state: State,
  previous_action: Action,
  opponent_previous_previous: Action,
}

impl Player for PlayerC {
  fn first_round(&self) -> Action {
    Action::Cooperate
  }
  fn next_round(&mut self, opponent_previous: &Action) -> Action {
    let action = match self.state {
      State::DefaultState => {
        match (
          &self.previous_action,
          opponent_previous,
          &self.opponent_previous_previous,
        ) {
          (Action::Cooperate, Action::Cooperate, Action::Cooperate) => {
            // Rule #2. Player is cooperating and opponent has cooperated for the previous two turns.
            self.state = State::DefectUntilPunished;
            Action::Defect
          }
          (Action::Defect, Action::Defect, _) => {
            // Rule #3. Both players are defecting.
            Action::Cooperate
          }
          (Action::Cooperate, Action::Defect, _) => {
            // Rule #4. Player is cooperating and opponent is defecting.
            self.state = State::TwoRoundPunish;
            Action::Defect
          }
          // Rule #1.
          _ => Action::Cooperate,
        }
      }
      State::TwoRoundPunish => {
        // Already punished the previous round when transitioning to this state. Punish one more time.
        self.state = State::DefaultState;
        Action::Defect
      }
      State::DefectUntilPunished => match opponent_previous {
        Action::Cooperate => Action::Defect,
        Action::Defect => {
          self.state = State::DefaultState;
          Action::Cooperate
        }
      },
    };
    self.opponent_previous_previous = opponent_previous.clone();
    self.previous_action = action.clone();
    action
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_two_round_punish() {
    let config = MatchConfig {
      num_rounds: 200,
      both_coop_points: 4,
      defect_against_coop_points: 7,
      coop_against_defect_points: 0,
      both_defect_points: 1,
    };
    let mut p = StrategyC::new().create_player(&config);
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
    let config = MatchConfig {
      num_rounds: 200,
      both_coop_points: 4,
      defect_against_coop_points: 7,
      coop_against_defect_points: 0,
      both_defect_points: 1,
    };
    let mut p = StrategyC::new().create_player(&config);
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
