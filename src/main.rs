mod strategies;
use strategies::Strategy;

fn main() {
    let strategies = strategies::get_strategies();
    let mut scores = vec![0; strategies.len()];
    introduce_strategies(&strategies);

    // Run through all possible pairs.
    for (s1_index, s1) in strategies.iter().enumerate() {
        for s2_index in s1_index + 1..strategies.len() {
            let s2 = &strategies[s2_index];
            let (score1, score2) = play_strategies(s1, s2);
            println!(
                "{} vs {} result: {} to {}",
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
    show_final_results(&strategies, &scores);
}

fn play_strategies(s1: &Box<dyn Strategy>, s2: &Box<dyn Strategy>) -> (u32, u32) {
    let mut p1 = s1.create_player();
    let mut p2 = s2.create_player();

    let a1 = p1.first_round();
    let a2 = p2.first_round();
    p1.next_round(&a2);
    p2.next_round(&a1);
    (0, 0)
}

fn introduce_strategies(strategies: &Vec<Box<dyn Strategy>>) {
    println!("Strategies:");
    for s in strategies.iter() {
        println!("{}:", s.name());
        println!("{}\n", s.description());
    }
}

fn show_final_results(strategies: &Vec<Box<dyn Strategy>>, scores: &Vec<u32>) {
    println!("Final scores:");
    for (i, s) in strategies.iter().enumerate() {
        println!("{}: {}", s.name(), scores[i]);
    }
}
