mod config;

use config::Game;

fn main() {
    let game = Game::new();

    let puzzle_input = std::fs::read_to_string("assets/input.txt").expect("Should contain the puzzle input.");
    let games = puzzle_input.split('\n').collect::<Vec<&str>>();

    let mut sum = 0;
    let mut sum_min_cubes = 0;
    for round in games {
        sum = sum + game.possible_game(&round.to_string());
        sum_min_cubes = sum_min_cubes + game.min_cubes_per_game(&round.to_string()).1;
    }

    println!("\n[SOLUTION] part1: {sum} part2: {sum_min_cubes}\n");
}
