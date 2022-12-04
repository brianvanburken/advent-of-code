use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Unable to read file");

    let letter_priorities = ('a'..='z')
        .chain('A'..='Z')
        .into_iter()
        .enumerate()
        .map(|(index, chr)| (chr, index + 1))
        .collect::<HashMap<char, usize>>();

    let part_1 = input
        .lines()
        .into_iter()
        .map(|line| {
            let compartment_length = line.len() / 2;
            let (compartment_a, compartment_b) = line.split_at(compartment_length);
            let matched_common_char = compartment_a
                .chars()
                .find(|&c| compartment_b.contains(c))
                .expect("Expected a common char in string");

            letter_priorities
                .get(&matched_common_char)
                .expect("Expected letter priority to be set for char")
        })
        .sum::<usize>();

    println!("Part 1: {:?}", part_1);

    let part_2 = input
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|chunk| {
            let [a, b, c] = chunk else {
                panic!("Expected chunk of 3")
            };
            let matched_common_char = a
                .chars()
                .find(|&chr| b.contains(chr) && c.contains(chr))
                .expect("Expected a common char in strings");

            *letter_priorities
                .get(&matched_common_char)
                .expect("Expected letter priority to be set for char")
        })
        .sum::<usize>();

    println!("Part 2: {:?}", part_2);
}
