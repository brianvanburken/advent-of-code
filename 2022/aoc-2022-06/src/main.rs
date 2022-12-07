use std::{collections::HashSet, fs};

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Unable to read file");

    let part_1 = find_marker(&input, 4);
    println!("Part 1: {:?}", part_1);

    let part_2 = find_marker(&input, 14);
    println!("Part 2: {:?}", part_2);
}

fn find_marker(input: &String, window: usize) -> usize {
    input
        .chars()
        .collect::<Vec<char>>()
        .windows(window)
        .position(|window| all_unique(window))
        .map(|position| position + window)
        .expect("Expected to find a marker!")
}

fn all_unique(chars: &[char]) -> bool {
    HashSet::<&char>::from_iter(chars.iter()).len() == chars.len()
}
