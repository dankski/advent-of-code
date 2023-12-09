use std::fs::File;
use std::io::{BufReader};

mod util;

fn main() {
    // let puzzle_input = std::fs::read_to_string("assets/input.txt").expect("Should contain the puzzle input.");
    let file = File::open("assets/test_input.txt").expect("Should contain the puzzle input.");
    let reader = BufReader::new(file);

    let sum = util::gears_sum(reader);

    // let sum = util::parts_sum(&puzzle_input);
    println!("\n[SOLUTION] part1: {sum}\n");
}