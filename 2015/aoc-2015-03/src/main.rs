use std::collections::HashSet;
use std::fs;

fn calculate_unique_delivered_houses(sequence: &str) -> usize {
    let mut current_x = 0;
    let mut current_y = 0;
    let start_coordinate = format!("{},{}", current_x, current_y);
    let mut visited_coordinates = HashSet::from([start_coordinate]);

    for movement in sequence.chars() {
        match movement {
            '>' => current_x += 1,
            '<' => current_x -= 1,
            '^' => current_y += 1,
            'v' => current_y -= 1,
            _ => (),
        }
        let coordinate = format!("{},{}", current_x, current_y);
        visited_coordinates.insert(coordinate);
    }
    visited_coordinates.len()
}

fn main() {
    let file = "./input.txt";
    let input = fs::read_to_string(file).unwrap();
    let all_visited_houses = input
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|sequence| calculate_unique_delivered_houses(sequence))
        .sum::<usize>();

    println!("Amount of visited houses: {}", all_visited_houses)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_returns_2_for_one_step_east() {
        assert_eq!(calculate_unique_delivered_houses(">"), 2);
    }

    #[test]
    fn it_returns_4_for_square() {
        assert_eq!(calculate_unique_delivered_houses("^>v<"), 4);
    }

    #[test]
    fn it_returns_2_for_repeated_same_houses() {
        assert_eq!(calculate_unique_delivered_houses("^v^v^v^v^v"), 2);
    }
}
