use std::fs;

// Part 1
fn calculate_floor(path: String) -> i32 {
    let upstairs = path.matches('(').count() as i32;
    let downstairs = path.matches(')').count() as i32;
    upstairs - downstairs
}

// Part 2
fn position_first_entering_basement(path: String) -> usize {
    let mut current_floor = 0;
    for (index, char) in path.chars().enumerate() {
        match char {
            '(' => current_floor += 1,
            ')' => current_floor -= 1,
            _ => (),
        }

        if current_floor < 0 {
            return index + 1;
        }
    }
    0
}

fn main() {
    let data = fs::read_to_string("./input.txt").expect("Unable to read file");
    let floor = calculate_floor(data.clone());
    let basement = position_first_entering_basement(data.clone());
    println!("Your final floor is: {}", floor);
    println!("First time entering basement: {}", basement);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_returns_to_floor_0() {
        assert_eq!(calculate_floor("(())".to_string()), 0);
        assert_eq!(calculate_floor("()()".to_string()), 0);
    }

    #[test]
    fn it_returns_to_floor_3() {
        assert_eq!(calculate_floor("(((".to_string()), 3);
        assert_eq!(calculate_floor("(()(()(".to_string()), 3);
        assert_eq!(calculate_floor("))(((((".to_string()), 3);
    }

    #[test]
    fn it_returns_to_basement() {
        assert_eq!(calculate_floor("())".to_string()), -1);
        assert_eq!(calculate_floor("))(".to_string()), -1);
    }

    #[test]
    fn it_returns_to_basement_3() {
        assert_eq!(calculate_floor(")())())".to_string()), -3);
        assert_eq!(calculate_floor(")))".to_string()), -3);
    }

    #[test]
    fn it_returns_to_first_entering_basement_at_1() {
        assert_eq!(position_first_entering_basement(")".to_string()), 1);
    }

    #[test]
    fn it_returns_to_first_entering_basement_at_5() {
        assert_eq!(position_first_entering_basement("()())".to_string()), 5);
    }
}
