use std::collections::hash_map::HashMap;

fn count_chars(box_id: &str) -> HashMap<char, i32> {
    let mut map = HashMap::new();
    for c in box_id.chars() {
        *map.entry(c).or_insert(0) += 1;
    }
    map
}

fn hamming_distance(str1: &str, str2: &str) -> i32 {
    // Assumes str1 and str2 are equal length

    str1.chars().zip(str2.chars()).map(|(c1, c2)| {
        if c1 == c2 {
            0
        } else {
            1
        }
    }).sum()
}

fn main() {
    let box_ids = std::fs::read_to_string("input.txt").expect("File missing");
    let (twos, threes) = box_ids.split_whitespace().map(|box_id| {
        let mut seen_double = 0;
        let mut seen_triple = 0;

        let map = count_chars(box_id);
        for (_, count) in map.iter() {
            if *count == 2 && seen_double == 0 {
                seen_double = 1;
            };
            if *count == 3 && seen_triple == 0{
                seen_triple = 1;
            };
        };
        (seen_double, seen_triple)
    }).fold((0,0), |acc, vals| {
        (acc.0 + vals.0, acc.1 + vals.1)
    });

    println!("Checksum is: {}", twos * threes);

    // For part 2, this seems to work as it seems like there are only
    // 2 box IDs with that differ in 1 character
    let box_id_vec: Vec<&str> = box_ids.split_whitespace().collect();
    for i in 0..box_id_vec.len() - 1{
        for j in 1..box_id_vec.len() {
            if hamming_distance(box_id_vec[i], box_id_vec[j]) == 1 {
                println!("{}", box_id_vec[i]);
            }
        }
    }
}
