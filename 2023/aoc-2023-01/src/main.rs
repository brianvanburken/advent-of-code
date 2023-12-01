use std::fs;

const DIGITS: [&[u8]; 9] = [
    b"one", b"two", b"three", b"four", b"five", b"six", b"seven", b"eight", b"nine",
];

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

    let mut part2 = 0;
    for line in input.lines() {
        part2 += convert_to_double_digit(line);
    }
    println!("Part 2: {}", part2);
}

fn convert_to_double_digit(i: &str) -> u32 {
    let mut first: u32 = 0;
    let mut last: u32 = 0;

    let chars = i.as_bytes();
    let mut i = 0;

    while i < chars.len() {
        let possible_digit = chars[i];

        if possible_digit.is_ascii_digit() {
            let digit = (possible_digit - b'0') as u32;
            first = if first == 0 { digit * 10 } else { first };
            last = digit;
        } else {
            for (index, spelled_digit) in DIGITS.iter().enumerate() {
                if chars[i..].starts_with(spelled_digit) {
                    let digit = (index + 1) as u32;
                    first = if first == 0 { digit * 10 } else { first };
                    last = digit;
                }
            }
        }
        i += 1;
    }

    first + last
}
