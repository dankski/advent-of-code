mod utils;
mod competition;

use utils::parse_races;

use crate::utils::error_margin;

fn main() {
    let races = parse_races(&"../assets/races.txt");

    println!("[SOLUTION]: {}", error_margin(&races));
}
