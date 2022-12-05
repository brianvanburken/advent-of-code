use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Unable to read file");
    part_1(&input);
}

fn part_1(input: &String) {
    let Some((towers_part, moves_part)) = input.split_once("\n\n") else {
        panic!("Expect to split between towers and moves")
    };
    let mut towers = parse_towers(towers_part);
    let moves = parse_moves(moves_part);

    for (amount, from, to) in moves.iter() {
        for _ in 0..*amount {
            let crate_to_move: char = towers[from - 1].pop().expect("Tower is empty");
            towers[to - 1].push(crate_to_move);
        }
    }

    let top_crates = towers
        .iter_mut()
        .map(|tower| tower.pop().unwrap_or(' '))
        .collect::<String>();

    println!("Part 1: {:?}", top_crates);
}

fn parse_towers(input: &str) -> Vec<Vec<char>> {
    let mut lines = input.lines().rev().into_iter();
    let numbers_line = lines.next().expect("Expected at least one line.");
    let tower_numbers_len = numbers_line.len() + 2; // +2 since we have a new line cut away by lines
    let amount_of_towers = tower_numbers_len / 4; // zero-indexed
    let mut towers = Vec::with_capacity(amount_of_towers);

    let length_of_tower_with_most_crates = input.lines().count() - 1;
    for _ in 0..amount_of_towers {
        towers.push(Vec::with_capacity(length_of_tower_with_most_crates));
    }

    for line in lines {
        // step by 4 so we step on char for each crate
        let mut crates = line.chars().skip(1).step_by(4);
        for tower in towers.iter_mut() {
            match crates.next() {
                None => (),
                Some(' ') => (),
                Some(c) => tower.push(c),
            }
        }
    }
    towers
}

fn parse_moves(input: &str) -> Vec<(usize, usize, usize)> {
    let mut moves = Vec::new();
    for line in input.lines() {
        moves.push(parse_move_line(line));
    }
    moves
}

fn parse_move_line(line: &str) -> (usize, usize, usize) {
    let mut split = line.split_whitespace();
    let amount = split
        .nth(1)
        .expect("Expected 'amount'")
        .parse::<usize>()
        .expect("Expected 'amount' as a number");
    let from = split
        .nth(1)
        .expect("Expected 'from'")
        .parse::<usize>()
        .expect("Expected 'from' as a number");
    let to = split
        .nth(1)
        .expect("Expected 'to'")
        .parse::<usize>()
        .expect("Expected 'to' as a number");
    (amount, from, to)
}
