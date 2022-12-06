use std::{collections::HashSet, fs};

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Unable to read file");

    let start_of_marker = input
        .chars()
        .collect::<Vec<char>>()
        .windows(4)
        .position(|window| HashSet::<&char>::from_iter(window.iter()).len() == window.len())
        .expect("Expected to find a marker!");

    println!("Part 1: {:?}", (start_of_marker + 4));
}
