mod strategies;
use strategies::Action;
use strategies::Strategy;

const NUM_ROUNDS: u32 = 200;

fn main() {
    let strategies = strategies::get_strategies();
    let mut scores = vec![0; strategies.len()];
    introduce_game(&strategies);

    // Run through all possible pairs.
    for (s1_index, s1) in strategies.iter().enumerate() {
        for s2_index in s1_index + 1..strategies.len() {
            let s2 = &strategies[s2_index];
            let (score1, score2) = play_strategies(s1, s2);
            println!(
                "{} vs {} results: {} to {}.",
                s1.name(),
                s2.name(),
                score1,
                score2
            );

            // Update the total scores.
            scores[s1_index] += score1;
            scores[s2_index] += score2;
        }
    }
    println!();

    show_final_results(&strategies, &scores);
}

fn play_strategies(s1: &Box<dyn Strategy>, s2: &Box<dyn Strategy>) -> (u32, u32) {
    let mut score1 = 0;
    let mut score2 = 0;
    let mut p1 = s1.create_player();
    let mut p2 = s2.create_player();

    let mut a1 = p1.first_round();
    let mut a2 = p2.first_round();
    let (x1, x2) = evaluate_actions(&a1, &a2);
    score1 += x1;
    score2 += x2;

    for _i in 0..NUM_ROUNDS - 1 {
        let temp1 = p1.next_round(&a2);
        let temp2 = p2.next_round(&a1);
        a1 = temp1;
        a2 = temp2;
        let (x1, x2) = evaluate_actions(&a1, &a2);
        score1 += x1;
        score2 += x2;
    }
    (score1, score2)
}

fn evaluate_actions(a1: &Action, a2: &Action) -> (u32, u32) {
    match (a1, a2) {
        (Action::Cooperate, Action::Cooperate) => (4, 4),
        (Action::Cooperate, Action::Defect) => (0, 7),
        (Action::Defect, Action::Cooperate) => (7, 0),
        (Action::Defect, Action::Defect) => (1, 1),
    }
}

fn introduce_game(strategies: &Vec<Box<dyn Strategy>>) {
    println!("Strategies:");
    for s in strategies.iter() {
        println!("{}:", s.name());
        println!("{}\n", s.description());
    }
    println!("Matches set to {} rounds.", NUM_ROUNDS);
}

fn show_final_results(strategies: &Vec<Box<dyn Strategy>>, scores: &Vec<u32>) {
    println!("Final scores:");
    for (i, s) in strategies.iter().enumerate() {
        println!("{}: {}", s.name(), scores[i]);
    }
}
