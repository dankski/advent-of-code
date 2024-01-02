mod almanac;

fn main() {
    let mut almanac_input = std::fs::read_to_string("assets/input.txt").expect("File input.txt exists");

    let solution = almanac::lowest_location(&almanac_input);

    println!("[SOLUTION][1] {}", solution);

    almanac_input = std::fs::read_to_string("assets/input.txt").expect("File input.txt exists");
    let solution = almanac::lowest_location_part_2(&almanac_input);
    
    println!("[SOLUTION][2] {}", solution);
}
