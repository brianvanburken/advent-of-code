use fancy_regex::Regex;
use std::fs;

fn is_nice_string(text: &str) -> bool {
    let other_than_vowels = Regex::new(r"(.*[aeiou]){3}").unwrap();
    let contains_bad_parts = Regex::new(r"(ab|cd|pq|xy)").unwrap();
    let contains_repeated_letter = Regex::new(r"(.)\1{1}").unwrap();

    return other_than_vowels.is_match(text).unwrap()
        && contains_repeated_letter.is_match(text).unwrap()
        && !contains_bad_parts.is_match(text).unwrap();
}

fn main() {
    let file = "./input.txt";
    let input = fs::read_to_string(file).unwrap();
    let amount_of_nice_strings = input
        .split("\n")
        .filter(|l| !l.is_empty() && is_nice_string(l))
        .collect::<Vec<_>>()
        .len();

    println!("Amount of nice strings: {}", amount_of_nice_strings);
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
}
