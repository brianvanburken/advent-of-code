use std::fs;

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Unable to read input file");

    let mut part1 = 0;
    let mut part2 = 0;
    for line in input.lines() {
        let game = Game::from(line);
        if game.is_possible() {
            part1 += game.id;
        }
        part2 += game.powers();
    }

    println!("Part 1: {}", part1);
    println!("Part 2: {}", part2);
}

#[derive(Debug)]
struct Game {
    id: u32,
    sets: Vec<GameSet>,
}
impl Game {
    fn is_possible(&self) -> bool {
        self.sets.iter().all(|set| set.is_possible())
    }

    fn powers(&self) -> u32 {
        let (red, green, blue) = self.sets.iter().fold((0, 0, 0), |(red, green, blue), set| {
            (red.max(set.red), green.max(set.green), blue.max(set.blue))
        });
        red as u32 * green as u32 * blue as u32
    }
}

#[derive(Debug, Default)]
struct GameSet {
    red: u8,
    green: u8,
    blue: u8,
}
impl GameSet {
    fn is_possible(&self) -> bool {
        self.red <= 12 && self.green <= 13 && self.blue <= 14
    }
}

impl From<&str> for Game {
    fn from(value: &str) -> Self {
        let (game, game_sets) = value
            .split_once(':')
            .expect("Expected a valid game line format");
        let id = game
            .split_once(' ')
            .expect("Expected format 'Game <id>'")
            .1
            .parse::<u32>()
            .expect("Expected an valid number for the Game ID");

        let mut sets: Vec<GameSet> = vec![];
        for set_string in game_sets.split(';') {
            let mut game_set = GameSet::default();

            for set_part in set_string.split(',') {
                let mut parts = set_part.split_whitespace();
                let amount = parts
                    .next()
                    .expect("Expected valid set")
                    .parse::<u8>()
                    .expect("Expected an amount of cubes");

                match parts.next() {
                    Some("red") => game_set.red += amount,
                    Some("green") => game_set.green += amount,
                    Some("blue") => game_set.blue += amount,
                    Some(_) => (),
                    None => (),
                }
            }

            sets.push(game_set);
        }

        Self { id, sets }
    }
}
