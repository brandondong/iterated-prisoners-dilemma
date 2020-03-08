use iterated_prisoners_dilemma::strategies::strategy_a::StrategyA;
use iterated_prisoners_dilemma::strategies::strategy_b::StrategyB;
use iterated_prisoners_dilemma::strategies::strategy_c::StrategyC;
use iterated_prisoners_dilemma::strategies::strategy_d::StrategyD;
use iterated_prisoners_dilemma::{
    aggregate_results, play_strategies, MatchConfig, Strategy, StrategyScore,
};

const MATCH_CONFIG: MatchConfig = MatchConfig {
    num_rounds: 200,
    both_coop_points: 4,
    defect_against_coop_points: 7,
    coop_against_defect_points: 0,
    both_defect_points: 1,
};

fn main() {
    let strategies: [Box<dyn Strategy>; 4] = [
        Box::new(StrategyA::new()),
        Box::new(StrategyB::new()),
        Box::new(StrategyC::new()),
        Box::new(StrategyD::new()),
    ];
    introduce_game(&strategies);

    let results = play_strategies(&strategies, &MATCH_CONFIG);
    for result in results.iter() {
        let p1 = &result.s1;
        let p2 = &result.s2;
        println!(
            "{} vs {} results: {} to {}.",
            p1.strategy.name(),
            p2.strategy.name(),
            p1.score,
            p2.score
        );
    }
    println!();

    let totals = aggregate_results(&results);
    show_final_results(&totals);
}

fn introduce_game(strategies: &[Box<dyn Strategy>]) {
    println!("Strategies:");
    for s in strategies {
        println!("{}:", s.name());
        println!("{}\n", s.description());
    }
    println!("Matches set to {} rounds.", MATCH_CONFIG.num_rounds);
}

fn show_final_results(totals: &[StrategyScore]) {
    println!("Final scores:");
    for result in totals {
        println!("{}: {}", result.strategy.name(), result.score);
    }
}
