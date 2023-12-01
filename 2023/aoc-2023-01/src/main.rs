use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Unable to read input file");

    let mut part1: u32 = 0;
    for line in input.lines() {
        let mut digits = line.chars().filter_map(|c| c.to_digit(10));
        let first = digits.next().expect("Expect to have at least one number");
        let last = digits.last().unwrap_or(first);
        part1 += first * 10 + last;
    }
    println!("Part 1: {}", part1);
}

