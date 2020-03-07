mod strategies;
use strategies::Strategy;

fn main() {
    let mut strategies = strategies::get_strategies();
    let mut scores = vec![0; strategies.len()];
    introduce_strategies(&strategies);

    // Run through all possible pairs.
    for split_index in 1..strategies.len() {
        let (left, right) = strategies.split_at_mut(split_index);
        let s1_index = split_index - 1;
        let s1 = &mut left[s1_index];
        for (i, s2) in right.iter_mut().enumerate() {
            let s2_index = split_index + i;
            let (score1, score2) = play_strategies(s1, s2);
            println!("{} vs {} result: {} to {}", s1, s2, score1, score2);

            // Update the total scores.
            let p1 = &mut scores[s1_index];
            *p1 += score1;
            let p2 = &mut scores[s2_index];
            *p2 += score2;

            // Reset any intermediate state before the next match.
            *s1 = s1.reset();
            *s2 = s2.reset();
        }
    }
    show_final_results(&strategies, &scores);
}

fn play_strategies(s1: &mut Box<dyn Strategy>, s2: &mut Box<dyn Strategy>) -> (u32, u32) {
    s1.first_round();
    s2.first_round();
    (0, 0)
}

fn introduce_strategies(strategies: &Vec<Box<dyn Strategy>>) {
    println!("Strategies:");
    for s in strategies.iter() {
        println!("{}:", s);
        println!("{}\n", s.description());
    }
}

fn show_final_results(strategies: &Vec<Box<dyn Strategy>>, scores: &Vec<u32>) {
    println!("Final scores:");
    for (i, s) in strategies.iter().enumerate() {
        println!("{}: {}", s, scores[i]);
    }
}
