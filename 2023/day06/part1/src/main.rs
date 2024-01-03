mod utils;
mod competition;

use utils::parse_races;

fn main() {
    let races = parse_races(&"../assets/races_example.txt");

    println!("Hello, world!");
}
