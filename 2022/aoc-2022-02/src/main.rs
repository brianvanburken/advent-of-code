use std::fs;

// X, A = Rock
// Y, B = Paper
// Z, C = Scissors
fn main() {
    let file = "./input.txt";
    let input = fs::read_to_string(file).unwrap();

    let mut total_score_part_1: usize = 0;
    let mut total_score_part_2: usize = 0;
    for game in input.lines() {
        let mut splitter = game.split_whitespace();
        let opponent = splitter
            .next()
            .expect("Expected opponent choice to be present");
        let player_or_strategy = splitter
            .next()
            .expect("Expected player choice to be present");

        total_score_part_1 += points_for_shape(player_or_strategy);
        total_score_part_1 += win_score(opponent, player_or_strategy);

        let player = win_draw_lose_shape(opponent, player_or_strategy);
        total_score_part_2 += points_for_shape(player);
        total_score_part_2 += win_score(opponent, player);
    }
    println!("Part 1: {:?}", total_score_part_1);
    println!("Part 2: {:?}", total_score_part_2);
}

// X = lose
// Y = draw
// Z = win
fn win_draw_lose_shape<'a>(opponent: &'a str, strategy: &'a str) -> &'a str {
    match (opponent, strategy) {
        // lose
        ("A", "X") => "Z",
        ("B", "X") => "X",
        ("C", "X") => "Y",
        // draw
        ("A", "Y") => "X",
        ("B", "Y") => "Y",
        ("C", "Y") => "Z",
        // win
        ("A", "Z") => "Y",
        ("B", "Z") => "Z",
        ("C", "Z") => "X",

        _ => panic!("Unknown input"),
    }
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
