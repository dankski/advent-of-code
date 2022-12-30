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

fn evaluate_floor(input: &Vec<char>) -> (i32, u32) {
    let mut floor: i32  = 0;
    let mut enters_the_basement_at_pos: u32 = 0;

    for (i, c) in input.iter().enumerate() {
        if *c == '(' {
            floor = floor + 1;
        } 

        if *c == ')' {
            floor = floor - 1;
        }

        if floor == -1 && enters_the_basement_at_pos == 0 {
            enters_the_basement_at_pos = i as u32 + 1;
        }
    }
    
    (floor, enters_the_basement_at_pos)
}

fn convert_input_vec (input: &String) -> Vec<char> {
    input.chars().collect()
}


fn main() {
    let puzzle_input = std::fs::read_to_string("assets/input.txt").expect("Should contain the puzzle input.");

    let result =  evaluate_floor(&convert_input_vec(&puzzle_input));
    println!("Puzzle One Answer is 280, and result is {}. First time he enters the basement: {}.", result.0, result.1);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_evaluate_floor_0 () {
        let v1 = vec!['(','(',')',')'];
        let v2 = vec![')','(','(',')'];

        assert_eq!(0, evaluate_floor(&v1).0);
        assert_eq!(0, evaluate_floor(&v2).0);
    }


    #[test]
    fn should_evaluate_floor_3 () {
        let v1 = vec!['(','(','('];
        let v2 = vec!['(','(',')','(','(',')','('];
        let v3 = vec![')',')','(','(','(','(','('];

        assert_eq!(3, evaluate_floor(&v1).0);
        assert_eq!(3, evaluate_floor(&v2).0);
        assert_eq!(3, evaluate_floor(&v3).0);
    }

    #[test]
    fn should_evaluate_to_minus_one () {

        let v1 = vec!['(',')',')'];
        let v2 = vec![')',')','('];

        assert_eq!(-1, evaluate_floor(&v1).0);
        assert_eq!(-1, evaluate_floor(&v2).0);
    }

    #[test]
    fn should_evaluate_to_minus_three () {

        let v1 = vec![')',')',')'];
        let v2 = vec![')','(',')',')','(',')',')'];

        assert_eq!(-3, evaluate_floor(&v1).0);
        assert_eq!(-3, evaluate_floor(&v2).0);
    }

    #[test]
    fn should_return_position_one () {
        let v1 = vec![')'];

        assert_eq!(1, evaluate_floor(&v1).1);
    }

    #[test]
    fn should_return_position_five () {
        let v1 = vec!['(',')','(',')',')'];

        assert_eq!(5, evaluate_floor(&v1).1);
    }


    #[test]
    fn should_convert_vec () {
        let input = String::from("(())");
        let v: Vec<char> = convert_input_vec(&input);
        
        assert_eq!(4, v.len());
    }
}
