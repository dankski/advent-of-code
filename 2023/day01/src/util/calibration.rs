
pub fn calibration_value(line: &String) -> u32 {

    if line.is_empty() {
        return 0;
    }

    let first_left = find_first_digit_from_left(line);
    let first_right = find_first_digit_from_right(line);

    let value = first_left.to_string() + &first_right.to_string();

    return value.parse::<u32>().unwrap();
}

fn find_first_digit_from_right(line: &str) -> char {
    for c in line.chars().rev() {
        if c.is_digit(10) {
            return c;
        }
    }
    return ' ';
}

fn find_first_digit_from_left(line: &str) -> char  {
    for c in line.chars().into_iter() {
        if c.is_digit(10) {
            return c;
        }
    }
    return ' ';
}


#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_return_12() {
        assert_eq!(calibration_value(&"1abc21abc2".to_owned()), 12);
    }

    #[test]
    fn should_return_38() {
        assert_eq!(calibration_value(&"pqr3stu8vwx".to_owned()), 38);
    }

    #[test]
    fn should_return_15() {
        assert_eq!(calibration_value(&"a1b2c3d4e5f".to_owned()), 15);
    }

    #[test]
    fn should_return_77() {
        assert_eq!(calibration_value(&"treb7uchet".to_owned()), 77);
    }

}