mod utils;
mod competition;

use utils::parse_race;

use crate::utils::error_margin;

fn main() {
    let race = parse_race(&"../assets/races.txt");

    println!("[SOLUTION]: {}", error_margin(&race));
}
