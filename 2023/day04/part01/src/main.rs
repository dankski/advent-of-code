mod scratch;
mod part02;

fn main() {
    let input = std::fs::read_to_string("assets/input.txt").expect("Should read test input");
    let total = scratch::winning_total(&input);

    println!("[SOLUTION][P1]: {total}");

    let input = std::fs::read_to_string("assets/input.txt").expect("Should read test input");
    let lines: Vec<&str> = input.trim().split('\n').collect();
    let total = part02::calculate_total_scratch_cards(lines);

    println!("[SOLUTION][P2]: {total}");
}

