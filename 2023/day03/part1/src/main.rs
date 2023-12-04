mod util;

fn main() {
    let puzzle_input = std::fs::read_to_string("assets/input.txt").expect("Should contain the puzzle input.");
    let sum = util::parts_sum(&puzzle_input);
    println!("\n[SOLUTION] part1: {sum}\n");
}