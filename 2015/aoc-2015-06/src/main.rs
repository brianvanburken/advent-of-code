use regex::Regex;
use std::fs;

fn run_light_instructions(
    mut lights: [[bool; 1000]; 1000],
    instructions: &str,
) -> [[bool; 1000]; 1000] {
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

    if instruction.is_empty() {
        return lights;
    }

    for row in &mut lights[start_x..=end_x] {
        for light in &mut row[start_y..=end_y] {
            *light = match instruction {
                "turn on" => true,
                "turn off" => false,
                "toggle" => !*light,
                _ => *light,
            }
        }
    }

    lights
}

fn calculate_lights(mut lights: [[bool; 1000]; 1000]) -> usize {
    let mut sum: usize = 0;
    for row in &mut lights[..] {
        for light in &mut row[..] {
            sum += if *light { 1 } else { 0 };
        }
    }
    sum
}

fn main() {
    let file = "./input.txt";
    let input = fs::read_to_string(file).unwrap();
    let lines = input.split("\n").filter(|l| !l.is_empty());

    let mut lights = [[false; 1000]; 1000];
    for line in lines {
        lights = run_light_instructions(lights, line);
    }

    dbg!(calculate_lights(lights));
}
