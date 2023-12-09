
use std::fs::File;
use std::io::{BufReader};

mod util;

fn main() {

    let file = File::open("assets/input.txt").expect("Should contain the puzzle input.");
    let reader = BufReader::new(file);

    let sum = util::gears_sum(reader);

    println!("\n[SOLUTION] part1: {sum}\n");
}