use std::fs;

fn main() {
    let data = fs::read_to_string("./input.txt")
        .expect("Unable to read input file")
        .lines()
        .map(|line| line.trim().parse::<u32>())
        .collect::<Result<Vec<_>, _>>()
        .expect("Unable to parse lines to u32");

    let mut increases = 0;
    let mut previous_measurement = data
        .first()
        .expect("Expected at least one value in the input file");


    for measurement in data.iter().skip(1) {
        if measurement > previous_measurement {
            increases += 1;
        }
        previous_measurement = measurement;
    }

    println!("Answer part 1: {}", increases);

    let window_size: usize = 3;
    let data_windows = data
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

    println!("Answer part 2: {}", data_windows);
}
