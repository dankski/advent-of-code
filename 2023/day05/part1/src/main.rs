mod almanac;

fn main() {
    let almanac = std::fs::read_to_string("assets/input.txt").expect("File input.txt exists");

    let solution = almanac::lowest_location(&almanac);

    println!("[SOLUTION][1] {}", solution);
}
