use regex::Regex;
use std::fs;

type Grid = [[u32; 1000]; 1000];

fn calculate_lights(lights: &Grid) -> usize {
    lights
        .iter()
        .map(|&row| row.iter().sum::<u32>() as usize)
        .sum()
}

fn main() {
    let file = "./input.txt";
    let input = fs::read_to_string(file).unwrap();
    let lines = input.split("\n").filter(|l| !l.is_empty());

    let mut lights: Grid = [[0; 1000]; 1000];
    let mut lights_2: Grid = [[0; 1000]; 1000];

    for instructions in lines {
        let match_instructions_regex =
            Regex::new(r"(.*) ([0-9]+),([0-9]+) through ([0-9]+),([0-9]+)").unwrap();
        let captures = match_instructions_regex.captures(instructions).unwrap();

        let instruction = captures.get(1).map_or("", |m| m.as_str());
        let start_x = captures
            .get(2)
            .map_or(0, |m| m.as_str().parse::<usize>().unwrap());
        let start_y = captures
            .get(3)
            .map_or(0, |m| m.as_str().parse::<usize>().unwrap());
        let end_x = captures
            .get(4)
            .map_or(0, |m| m.as_str().parse::<usize>().unwrap());
        let end_y = captures
            .get(5)
            .map_or(0, |m| m.as_str().parse::<usize>().unwrap());

        for x in start_x..=end_x {
            for y in start_y..=end_y {
                // Part 1
                lights[x][y] = match instruction {
                    "turn on" => 1,
                    "turn off" => 0,
                    "toggle" => {
                        if lights[y][x] == 0 {
                            1
                        } else {
                            0
                        }
                    }
                    _ => lights[y][x],
                };

                // Part 2
                lights_2[x][y] = match instruction {
                    "turn on" => lights_2[x][y] + 1,
                    "turn off" => {
                        if lights_2[x][y] <= 1 {
                            0
                        } else {
                            lights_2[x][y] - 1
                        }
                    }
                    "toggle" => lights_2[x][y] + 2,
                    _ => lights_2[x][y],
                };
            }
        }
    }

    println!("Lights part 1: {}", calculate_lights(&lights));
    println!("Lights part 2: {}", calculate_lights(&lights_2));
}
