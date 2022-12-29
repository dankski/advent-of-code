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
        } else {
            floor = floor - 1;
        }
    }
    
    floor
}



fn main() {
    println!("Hello, world!");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_evaluate_floor_0 () {
        let v1 = vec!['(','(',')',')'];
        let v2 = vec!['(',')','(',')'];

        assert_eq!(0, evaluate_floor(&v1));
        assert_eq!(0, evaluate_floor(&v2));
    }


    #[test]
    fn should_evaluate_floor_3 () {
        let v1 = vec!['(','(','('];
        let v2 = vec!['(','(',')','(','(',')','('];

        assert_eq!(3, evaluate_floor(&v1));
        assert_eq!(3, evaluate_floor(&v2));
    }
}

