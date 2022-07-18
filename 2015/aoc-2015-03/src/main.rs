use std::collections::HashSet;
use std::fs;

// Part 1
fn calculate_unique_delivered_houses(sequence: &str) -> usize {
    let mut pos = (0, 0);
    let mut visited_coordinates = HashSet::from([pos]);

    for movement in sequence.chars() {
        match movement {
            '^' => pos = (pos.0, pos.1 - 1),
            '>' => pos = (pos.0 + 1, pos.1),
            '<' => pos = (pos.0 - 1, pos.1),
            'v' => pos = (pos.0, pos.1 + 1),
            _ => (),
        }
        visited_coordinates.insert(pos);
    }
    visited_coordinates.len()
}

// Part 2
fn calculate_robo_and_santa_deliveries(sequence: &str) -> usize {
    let mut pos = [(0, 0), (0, 0)];
    let mut visited_coordinates = HashSet::from([pos[0]]);

    for (i, movement) in sequence.chars().enumerate() {
        match movement {
            '^' => pos[i % 2] = (pos[i % 2].0, pos[i % 2].1 - 1),
            '>' => pos[i % 2] = (pos[i % 2].0 + 1, pos[i % 2].1),
            '<' => pos[i % 2] = (pos[i % 2].0 - 1, pos[i % 2].1),
            'v' => pos[i % 2] = (pos[i % 2].0, pos[i % 2].1 + 1),
            _ => (),
        }
        visited_coordinates.insert(pos[i % 2]);
    }
    visited_coordinates.len()
}

fn main() {
    let file = "./input.txt";
    let input = fs::read_to_string(file).unwrap();
    let sequence = input.trim();

    let all_visited_houses = calculate_unique_delivered_houses(sequence);
    println!("Amount of visited houses by santa: {}", all_visited_houses);

    let both_visited_houses = calculate_robo_and_santa_deliveries(sequence);
    println!(
        "Amount of visited houses by santa and robo: {}",
        both_visited_houses
    );
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

    // Part 2
    #[test]
    fn it_returns_3_for_2_steps() {
        assert_eq!(calculate_robo_and_santa_deliveries("^v"), 3);
    }

    #[test]
    fn it_returns_3_for_both_returning_to_same_house() {
        assert_eq!(calculate_robo_and_santa_deliveries("^>v<"), 3);
    }

    #[test]
    fn it_returns_11_for_both_going_other_direction() {
        assert_eq!(calculate_robo_and_santa_deliveries("^v^v^v^v^v"), 11);
    }
}
