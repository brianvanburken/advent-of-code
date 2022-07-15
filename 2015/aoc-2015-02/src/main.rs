use std::fs;
use std::ops::Add;

fn calculate_total_square_of_wrapping_paper(l: usize, w: usize, h: usize) -> usize {
    let package_surfaces = vec![(l * w), (w * h), (h * l)];
    let slack = match package_surfaces.iter().min() {
        Some(smallest_surface) => *smallest_surface,
        None => 0,
    };
    return package_surfaces
        .iter()
        .map(|surface| 2 * surface)
        .sum::<usize>()
        .add(slack);
}

// This function ensures it always returns the 3 dimensions and will fill in gaps
// with zero.
fn parse_dimensions(dimensions: &str) -> [usize; 3] {
    let result = dimensions
        .split('x')
        .filter(|d| !d.is_empty())
        .take(3)
        .map(|n| n.parse::<usize>().unwrap())
        .collect::<Vec<_>>();

    return match result[..] {
        [] => [0, 0, 0],
        [l] => [l, 0, 0],
        [l, w] => [l, w, 0],
        [l, w, h] => [l, w, h],
        [l, w, h, ..] => [l, w, h],
    };
}

fn main() {
    let file = "./input.txt";
    let input = fs::read_to_string(file).unwrap();
    let total = input
        .split("\n")
        .filter(|l| !l.is_empty())
        .map(|line| {
            let [l, w, h] = parse_dimensions(line);
            calculate_total_square_of_wrapping_paper(l, w, h) as i32
        })
        .sum::<i32>();

    println!("Total wrapping paper surface needed: {}", total);
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn it_returns_58_for_2_by_3_by_4() {
        assert_eq!(calculate_total_square_of_wrapping_paper(2, 3, 4), 58);
    }

    #[test]
    fn it_returns_43_for_1_by_1_by_10() {
        assert_eq!(calculate_total_square_of_wrapping_paper(1, 1, 10), 43);
    }

    // parse_dimensions
    #[test]
    fn it_returns_three_zeroes_for_empty_string() {
        assert_eq!(parse_dimensions(""), [0, 0, 0]);
    }

    #[test]
    fn it_returns_length_if_only_present() {
        assert_eq!(parse_dimensions("2"), [2, 0, 0]);
    }

    #[test]
    fn it_returns_length_and_width() {
        assert_eq!(parse_dimensions("2x3"), [2, 3, 0]);
    }

    #[test]
    fn it_returns_length_and_width_and_height() {
        assert_eq!(parse_dimensions("2x3x4"), [2, 3, 4]);
    }

    #[test]
    fn it_ignores_anything_past_dimensions() {
        assert_eq!(parse_dimensions("2x3x4x5"), [2, 3, 4]);
    }
}
