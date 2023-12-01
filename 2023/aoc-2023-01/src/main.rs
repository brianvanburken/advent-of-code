use std::fs;

const DIGITS: [&[u8]; 9] = [
    b"one", b"two", b"three", b"four", b"five", b"six", b"seven", b"eight", b"nine",
];

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Unable to read input file");

    let mut part1 = 0;
    let mut part2 = 0;
    for line in input.lines() {
        part1 += line_to_digit(line);
        part2 += spelled_line_to_digit(line);
    }

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

fn line_to_digit(line: &str) -> u32 {
    let mut digits = line.chars().filter_map(|c| c.to_digit(10));
    let first = digits.next().unwrap_or(0);
    let last = digits.last().unwrap_or(first);
    first * 10 + last
}

fn spelled_line_to_digit(i: &str) -> u32 {
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
            for (zero_based_index, spelled_out_digit) in DIGITS.iter().enumerate() {
                if chars[i..].starts_with(spelled_out_digit) {
                    let digit = (zero_based_index + 1) as u32;
                    first = if first == 0 { digit * 10 } else { first };
                    last = digit;
                }
            }
        }
        i += 1;
    }

    first + last
}

#[cfg(test)]
mod test {
    use super::*;

    // Part 1
    #[test]
    fn line_to_digit_returns_0_if_no_digits_found() {
        assert_eq!(line_to_digit(""), 0);
        assert_eq!(line_to_digit(" "), 0);
        assert_eq!(line_to_digit("-"), 0);
        assert_eq!(line_to_digit("a"), 0);
        assert_eq!(line_to_digit("aa"), 0);
    }

    #[test]
    fn line_to_digit_returns_first_digit_as_last_with_only_a_single_digit() {
        assert_eq!(line_to_digit("7"), 77);
        assert_eq!(line_to_digit("1"), 11);
        assert_eq!(line_to_digit("3"), 33);
    }

    #[test]
    fn line_to_digit_returns_digit_if_only_contains_two_digits() {
        assert_eq!(line_to_digit("11"), 11);
        assert_eq!(line_to_digit("21"), 21);
        assert_eq!(line_to_digit("99"), 99);
    }

    #[test]
    fn line_to_digit_returns_digit_if_contains_two_digits_with_letters() {
        assert_eq!(line_to_digit("a99"), 99);
        assert_eq!(line_to_digit("99a"), 99);
        assert_eq!(line_to_digit("9a9"), 99);
        assert_eq!(line_to_digit("a9a9"), 99);
        assert_eq!(line_to_digit("9a9a"), 99);
        assert_eq!(line_to_digit("a9a9a"), 99);
    }

    #[test]
    fn line_to_digit_returns_first_and_last_if_contains_multiple_digits() {
        assert_eq!(line_to_digit("101"), 11);
        assert_eq!(line_to_digit("123"), 13);
        assert_eq!(line_to_digit("90000009"), 99);
    }

    #[test]
    fn line_to_digit_returns_first_and_last_if_contains_multiple_digits_with_letters() {
        assert_eq!(line_to_digit("a919"), 99);
        assert_eq!(line_to_digit("919a"), 99);
        assert_eq!(line_to_digit("9a19"), 99);
        assert_eq!(line_to_digit("a9a19"), 99);
        assert_eq!(line_to_digit("9a19a"), 99);
        assert_eq!(line_to_digit("a9a19a"), 99);
        assert_eq!(line_to_digit("a9a1a9a"), 99);
    }

    #[test]
    fn example_part1() {
        assert_eq!(line_to_digit("1abc2"), 12);
        assert_eq!(line_to_digit("pqr3stu8vwx"), 38);
        assert_eq!(line_to_digit("a1b2c3d4e5f"), 15);
        assert_eq!(line_to_digit("treb7uchet"), 77);
    }

    // Part 2
    #[test]
    fn spelled_line_to_digit_returns_0_if_not_found() {
        assert_eq!(line_to_digit(""), 0);
        assert_eq!(line_to_digit("a"), 0);
        assert_eq!(line_to_digit("-"), 0);
        assert_eq!(line_to_digit("zero"), 0);
    }

    #[test]
    fn example_part2() {
        assert_eq!(spelled_line_to_digit("two1nine"), 29);
        assert_eq!(spelled_line_to_digit("eightwothree"), 83);
        assert_eq!(spelled_line_to_digit("abcone2threexyz"), 13);
        assert_eq!(spelled_line_to_digit("xtwone3four"), 24);
        assert_eq!(spelled_line_to_digit("4nineeightseven2"), 42);
        assert_eq!(spelled_line_to_digit("zoneight234"), 14);
        assert_eq!(spelled_line_to_digit("7pqrstsixteen"), 76);
    }
}
