use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Unable to read file");

    let letter_priorities = ('a'..='z')
        .chain('A'..='Z')
        .into_iter()
        .enumerate()
        .map(|(index, chr)| (chr, index + 1))
        .collect::<HashMap<char, usize>>();

    let mut total_score = 0;
    for line in input.lines().into_iter() {
        let compartment_length = line.len() / 2;
        let compartment_a = &line[0..compartment_length];
        let compartment_b = &line[compartment_length..];

        let matched_common_char = compartment_a
            .chars()
            .find(|&c| compartment_b.contains(c))
            .expect("Expected a common char in string");

        total_score += letter_priorities
            .get(&matched_common_char)
            .expect("Expected letter priority to be set for char");
    }
    println!("Part 1: {:?}", total_score);
}
