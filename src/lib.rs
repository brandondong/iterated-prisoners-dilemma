pub mod strategies;
use std::cmp::min;
use std::collections::HashMap;
use std::fmt;
use std::hash::{Hash, Hasher};

#[derive(Clone, Debug, PartialEq)]
pub enum Action {
  Cooperate,
  Defect,
}

impl fmt::Display for Action {
  fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
    let s = match self {
      Action::Cooperate => "C",
      Action::Defect => "D",
    };
    write!(f, "{}", s)
  }
}

pub trait Strategy {
  fn name(&self) -> &str;
  fn description(&self) -> &str;
  fn create_player<'a>(&self, config: &'a MatchConfig) -> Box<dyn Player + 'a>;
  fn is_mixed(&self) -> bool;
}

impl<'a> PartialEq for dyn Strategy + 'a {
  fn eq(&self, other: &Self) -> bool {
    self.description() == other.description()
  }
}

impl<'a> Eq for dyn Strategy + 'a {}

impl<'a> Hash for dyn Strategy + 'a {
  fn hash<H: Hasher>(&self, state: &mut H) {
    self.description().hash(state);
  }
}

pub trait Player {
  fn first_round(&self) -> Action;
  fn next_round(&mut self, opponent_previous: &Action) -> Action;
}

pub struct MatchupResult<'a> {
  pub s1: StrategyMatchupResult<'a>,
  pub s2: StrategyMatchupResult<'a>,
}

pub struct StrategyMatchupResult<'a> {
  pub strategy: &'a dyn Strategy,
  pub score: u32,
  pub sample_match_history: Vec<Action>,
}

pub struct StrategyScore<'a> {
  pub strategy: &'a dyn Strategy,
  pub total_score: u32,
}

pub struct MatchConfig {
  pub num_rounds: u32,
  pub both_coop_points: u32,
  pub defect_against_coop_points: u32,
  pub coop_against_defect_points: u32,
  pub both_defect_points: u32,
}

pub fn play_strategies<'a>(
  strategies: &'a [Box<dyn Strategy>],
  config: &MatchConfig,
) -> Vec<MatchupResult<'a>> {
  let num_strategies = strategies.len();
  let mut results = Vec::with_capacity(num_strategies * (num_strategies - 1) / 2);

  for (s1_index, s1) in strategies.iter().enumerate() {
    for s2_index in s1_index + 1..num_strategies {
      let s2 = &strategies[s2_index];
      let result = play_strategy_pair(s1.as_ref(), s2.as_ref(), config);
      results.push(result);
    }
  }
  results
}

pub fn aggregate_results<'a>(results: &[MatchupResult<'a>]) -> Vec<StrategyScore<'a>> {
  let mut map = HashMap::new();
  for result in results.iter().flat_map(|r| vec![&r.s1, &r.s2]) {
    let score = map.entry(result.strategy).or_insert(0);
    *score += result.score;
  }
  let mut scores: Vec<StrategyScore> = map
    .into_iter()
    .map(|e| StrategyScore {
      strategy: e.0,
      total_score: e.1,
    })
    .collect();
  scores.sort_by(|a, b| b.total_score.cmp(&a.total_score));
  scores
}

fn play_strategy_pair<'a>(
  s1: &'a dyn Strategy,
  s2: &'a dyn Strategy,
  config: &MatchConfig,
) -> MatchupResult<'a> {
  let mut score1 = 0;
  let mut score2 = 0;
  let match_hist_len = min(config.num_rounds, 10) as usize;
  let mut actions1 = Vec::with_capacity(match_hist_len);
  let mut actions2 = Vec::with_capacity(match_hist_len);

  let num_runs = if s1.is_mixed() || s2.is_mixed() {
    100
  } else {
    1
  };
  for _i in 0..num_runs {
    let mut p1 = s1.create_player(config);
    let mut p2 = s2.create_player(config);

    let mut a1 = p1.first_round();
    let mut a2 = p2.first_round();
    update_scores(&mut score1, &mut score2, &a1, &a2, config);
    update_match_history(&mut actions1, &mut actions2, &a1, &a2, match_hist_len);

    for _i in 0..config.num_rounds - 1 {
      let temp1 = p1.next_round(&a2);
      let temp2 = p2.next_round(&a1);
      a1 = temp1;
      a2 = temp2;
      update_scores(&mut score1, &mut score2, &a1, &a2, config);
      update_match_history(&mut actions1, &mut actions2, &a1, &a2, match_hist_len);
    }
  }
  MatchupResult {
    s1: StrategyMatchupResult {
      strategy: s1,
      score: score1 / num_runs,
      sample_match_history: actions1,
    },
    s2: StrategyMatchupResult {
      strategy: s2,
      score: score2 / num_runs,
      sample_match_history: actions2,
    },
  }
}

fn update_match_history(
  actions1: &mut Vec<Action>,
  actions2: &mut Vec<Action>,
  a1: &Action,
  a2: &Action,
  match_hist_len: usize,
) {
  if actions1.len() < match_hist_len {
    actions1.push(a1.clone());
    actions2.push(a2.clone());
  }
}

fn update_scores(
  score1: &mut u32,
  score2: &mut u32,
  a1: &Action,
  a2: &Action,
  config: &MatchConfig,
) {
  let (x1, x2) = evaluate_actions(a1, a2, config);
  *score1 += x1;
  *score2 += x2;
}

fn evaluate_actions(a1: &Action, a2: &Action, c: &MatchConfig) -> (u32, u32) {
  match (a1, a2) {
    (Action::Cooperate, Action::Cooperate) => (c.both_coop_points, c.both_coop_points),
    (Action::Cooperate, Action::Defect) => {
      (c.coop_against_defect_points, c.defect_against_coop_points)
    }
    (Action::Defect, Action::Cooperate) => {
      (c.defect_against_coop_points, c.coop_against_defect_points)
    }
    (Action::Defect, Action::Defect) => (c.both_defect_points, c.both_defect_points),
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  use crate::strategies::strategy_a::StrategyA;
  use crate::strategies::strategy_d::StrategyD;

  #[test]
  fn test_nice_strategies_mixed() {
    let config = MatchConfig {
      num_rounds: 200,
      both_coop_points: 4,
      defect_against_coop_points: 7,
      coop_against_defect_points: 0,
      both_defect_points: 1,
    };
    let strategies: [Box<dyn Strategy>; 2] =
      [Box::new(StrategyA::new()), Box::new(StrategyD::new())];
    let results = play_strategies(&strategies, &config);
    assert_eq!(results.len(), 1);
    let result = &results[0];
    assert_eq!(result.s1.score, 800);
    assert_eq!(result.s2.score, 800);
    assert_eq!(
      result.s1.sample_match_history.iter().next(),
      Some(&Action::Cooperate)
    );
  }
}
