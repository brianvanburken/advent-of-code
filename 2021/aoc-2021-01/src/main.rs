use std::fs;

fn main() {
    let data = fs::read_to_string("./input.txt")
        .expect("Unable to read input file")
        .lines()
        .map(|line| line.trim().parse::<u32>())
        .collect::<Result<Vec<_>, _>>()
        .expect("Unable to parse lines to u32");

    let part_1 = data
        .windows(2)
        .fold(0, |acc, window| 
            match window {
                [a, b] if a < b => acc + 1,
                _ => acc,
            }
        );

    println!("Answer part 1: {}", part_1);

    let window_size: usize = 3;
    let part_2 = data
            .windows(window_size)
            .map(|window: &[u32]| window.iter().sum::<u32>())
            .collect::<Vec<u32>>()
            .windows(2)
            .fold(0, |acc, window| 
                match window {
                    [a, b] if a < b => acc + 1,
                    _ => acc,
                }
            );

    println!("Answer part 2: {}", part_2);
}
