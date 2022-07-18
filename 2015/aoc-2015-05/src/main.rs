use fancy_regex::Regex;
use std::fs;

// Part 1
fn is_nice_string(text: &str) -> bool {
    let other_than_vowels = Regex::new(r"(.*[aeiou]){3}").unwrap();
    let contains_bad_parts = Regex::new(r"(ab|cd|pq|xy)").unwrap();
    let contains_repeated_letter = Regex::new(r"(.)\1{1}").unwrap();

    return other_than_vowels.is_match(text).unwrap()
        && contains_repeated_letter.is_match(text).unwrap()
        && !contains_bad_parts.is_match(text).unwrap();
}

// Part 2
fn is_nice_string_2(text: &str) -> bool {
    let contains_repeated_letter_with_break = Regex::new(r"(.{1}).{1}(\1)").unwrap();
    let contains_repeated_pair = Regex::new(r"(.{2}).*\1{1,}").unwrap();

    return contains_repeated_letter_with_break.is_match(text).unwrap()
        && contains_repeated_pair.is_match(text).unwrap();
}

fn main() {
    let file = "./input.txt";
    let input = fs::read_to_string(file).unwrap();
    let lines = input.split("\n").filter(|l| !l.is_empty());

    let amount_of_nice_strings = lines
        .clone()
        .filter(|l| is_nice_string(l))
        .collect::<Vec<_>>()
        .len();

    println!("Amount of nice strings: {}", amount_of_nice_strings);

    let amount_of_nice_strings_2 = lines
        .clone()
        .filter(|l| is_nice_string_2(l))
        .collect::<Vec<_>>()
        .len();

    println!(
        "Amount of nice strings part 2: {}",
        amount_of_nice_strings_2
    );
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_returns_true_if_contains_three_vowels() {
        assert!(is_nice_string("ugknbfddgicrmopn"));
    }

    #[test]
    fn it_returns_true_for_aaa() {
        assert!(is_nice_string("aaa"));
    }

    #[test]
    fn it_returns_false_for_string_without_double_letter() {
        assert!(!is_nice_string("jchzalrnumimnmhp"));
    }

    #[test]
    fn it_returns_false_for_string_with_xy() {
        assert!(!is_nice_string("haegwjzuvuyypxyu"));
    }

    #[test]
    fn it_returns_false_for_string_with_only_one_vowel() {
        assert!(!is_nice_string("dvszwmarrgswjxmb"));
    }

    // Part 2
    #[test]
    fn it_returns_true_for_repeated_pair() {
        assert!(is_nice_string_2("qjhvhtzxzqqjkmpb"));
    }

    #[test]
    fn it_returns_true_for_letter_with_break() {
        assert!(is_nice_string_2("xxyxx"));
    }

    #[test]
    fn it_returns_false_no_repeated_letter() {
        assert!(!is_nice_string_2("uurcxstgmygtbstg"));
    }

    #[test]
    fn it_returns_false_no_repeated_pair() {
        assert!(!is_nice_string_2("ieodomkazucvgmuy"));
    }
}
