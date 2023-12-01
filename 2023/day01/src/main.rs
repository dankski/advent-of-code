mod util;

fn main() {

    let puzzle_input = std::fs::read_to_string("assets/input.txt").expect("Should contain the puzzle input.");
    let dims = input.split('\n').collect::<Vec<&str>>();
    let v = util::calibration_value(&"1abc21abc2".to_string());
    println!("[SOLUTION] {v}");
}
