mod config;

use config::Game;

fn main() {
    let game = Game::new();

    let puzzle_input = std::fs::read_to_string("assets/input.txt").expect("Should contain the puzzle input.");
    let games = puzzle_input.split('\n').collect::<Vec<&str>>();

    let mut sum = 0;
    for round in games {
        sum = sum + game.possible_game(&round.to_string()).0;
    }

    println!("\n[SOLUTION] {sum}\n");
}
