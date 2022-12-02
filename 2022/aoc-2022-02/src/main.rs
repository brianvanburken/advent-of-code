use std::fs;

// X, A = Rock
// Y, B = Paper
// Z, C = Scissors
fn main() {
    let file = "./input.txt";
    let input = fs::read_to_string(file).unwrap();

    let mut total_score: usize = 0;
    for game in input.lines() {
        let mut splitter = game.split_whitespace();
        let opponent = splitter
            .next()
            .expect("Expected opponent choice to be present");
        let player = splitter
            .next()
            .expect("Expected player choice to be present");
        total_score += points_for_shape(player);
        total_score += win_score(opponent, player);
    }
    println!("Part 1: {:?}", total_score);
}

fn points_for_shape(shape: &str) -> usize {
    match shape {
        "X" => 1,
        "Y" => 2,
        "Z" => 3,
        _ => 0,
    }
}

fn win_score(opponent: &str, player: &str) -> usize {
    match (opponent, player) {
        ("A", "Y") => 6,
        ("B", "Z") => 6,
        ("C", "X") => 6,
        ("A", "X") => 3,
        ("B", "Y") => 3,
        ("C", "Z") => 3,
        _ => 0,
    }
}
