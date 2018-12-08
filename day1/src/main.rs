use std::collections::BTreeSet;
use std::fs;

fn main() {
    let freqs = fs::read_to_string("input.txt").expect("input.txt file missing");
    let res: i32 = freqs
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .sum();

    println!("First part of the challenge: {}", res);

    // For the second part of the challenge, we'll use .cycle() on the iterator
    // to create an infinite iterator
    let mut it = freqs
        .split_whitespace()
        .map(|x| x.parse::<i32>().unwrap())
        .cycle();

    let mut seen_results: BTreeSet<i32> = BTreeSet::new();
    let mut current_freq = 0;

    while !seen_results.contains(&current_freq) {
        seen_results.insert(current_freq);
        let next_freq = it.next().unwrap();
        current_freq += next_freq;
    }

    println!("First frequency to repeat: {}", current_freq);
}
