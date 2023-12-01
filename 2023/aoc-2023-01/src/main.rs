use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Unable to read input file");

    let part1: u32 = input.lines().map(convert_to_double_digit).sum();

    println!("Part 1: {}", part1);
}

fn convert_to_double_digit(line: &str) -> u32 {
    let numbers = line
        .chars()
        .filter(char::is_ascii_digit)
        .collect::<String>();

    let digit = format!(
        "{}{}",
        numbers
            .chars()
            .next()
            .expect("Expect to have at least one number"),
        numbers
            .chars()
            .last()
            .expect("Expect to have a last number")
    );

    digit.parse::<u32>().expect("Expect a valid digit")
}
