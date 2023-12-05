mod scratch;

fn main() {
    let input = std::fs::read_to_string("assets/input.txt").expect("Should read test input");
    let total = scratch::winning_total(&input);

    println!("[SOLUTION]: {total}");
}
