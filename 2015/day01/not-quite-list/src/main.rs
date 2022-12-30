/// For example:
/// 
///     (()) and ()() both result in floor 0.
///     ((( and (()(()( both result in floor 3.
///     ))((((( also results in floor 3.
///     ()) and ))( both result in floor -1 (the first basement level).
///     ))) and )())()) both result in floor -3.
/// 
/// To what floor do the instructions take Santa?
/// 

fn evaluate_floor(input: &Vec<char>) -> i32 {
    let mut floor = 0 as i32;
    for c in input {
        if *c == '(' {
            floor = floor + 1;
        } 

        if *c == ')' {
            floor = floor - 1;
        }
    }
    
    floor
}

fn convert_input_vec (input: &String) -> Vec<char> {
    input.chars().collect()
}


fn main() {
    let puzzle_input = std::fs::read_to_string("assets/input.txt").expect("Should contain the puzzle input.");
    println!("Puzzle One Answer is 280, and result is {}", evaluate_floor(&convert_input_vec(&puzzle_input)));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_evaluate_floor_0 () {
        let v1 = vec!['(','(',')',')'];
        let v2 = vec![')','(','(',')'];

        assert_eq!(0, evaluate_floor(&v1));
        assert_eq!(0, evaluate_floor(&v2));
    }


    #[test]
    fn should_evaluate_floor_3 () {
        let v1 = vec!['(','(','('];
        let v2 = vec!['(','(',')','(','(',')','('];
        let v3 = vec![')',')','(','(','(','(','('];

        assert_eq!(3, evaluate_floor(&v1));
        assert_eq!(3, evaluate_floor(&v2));
        assert_eq!(3, evaluate_floor(&v3));
    }

    #[test]
    fn should_evaluate_to_minus_one () {

        let v1 = vec!['(',')',')'];
        let v2 = vec![')',')','('];

        assert_eq!(-1, evaluate_floor(&v1));
        assert_eq!(-1, evaluate_floor(&v2));
    }

    #[test]
    fn should_evaluate_to_minus_three () {

        let v1 = vec![')',')',')'];
        let v2 = vec![')','(',')',')','(',')',')'];

        assert_eq!(-3, evaluate_floor(&v1));
        assert_eq!(-3, evaluate_floor(&v2));
    }

    #[test]
    fn should_convert_vec () {
        let input = String::from("(())");
        let v: Vec<char> = convert_input_vec(&input);
        
        assert_eq!(4, v.len());
    }
}
