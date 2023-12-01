mod util;
fn main() {
    let puzzle_input = std::fs::read_to_string("assets/input.txt").expect("Should contain the puzzle input.");
    let calibrations = puzzle_input.split('\n').collect::<Vec<&str>>();

    let mut sum: u32 = 0;

    for calibration_value in calibrations {
        let v = util::calibration_value(&calibration_value.to_string());
  
        sum = sum + v.parse::<u32>().unwrap();
    }
   
    println!("\n[SOLUTION] {sum}\n");

}
