use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Unable to read file");

    let letter_priorities = ('a'..='z')
        .chain('A'..='Z')
        .into_iter()
        .enumerate()
        .map(|(index, chr)| (chr, index + 1))
        .collect::<HashMap<char, usize>>();

    let mut total_score_part_1 = 0;
    for line in input.lines().into_iter() {
        let compartment_length = line.len() / 2;
        let compartment_a = &line[0..compartment_length];
        let compartment_b = &line[compartment_length..];

        let matched_common_char = compartment_a
            .chars()
            .find(|&c| compartment_b.contains(c))
            .expect("Expected a common char in string");

        total_score_part_1 += letter_priorities
            .get(&matched_common_char)
            .expect("Expected letter priority to be set for char");
    }
    println!("Part 1: {:?}", total_score_part_1);

    let mut total_score_part_2 = 0;
    let mut lines = Vec::with_capacity(3);
    for line in input.lines() {
        lines.push(line);
        if lines.len() == 3 {
            let matched_common_char = lines[0]
                .chars()
                .find(|&c| lines[1].contains(c) && lines[2].contains(c))
                .expect("Expected a common char in strings");

            total_score_part_2 += letter_priorities
                .get(&matched_common_char)
                .expect("Expected letter priority to be set for char");
            lines.clear();
        }
    }
    println!("Part 2: {:?}", total_score_part_2);
}
