

pub fn calibration_value(line: &str) -> String {

    let first_left = first_value_from_left(line);
    let first_right = first_value_from_right(line);

    format!("{first_left}{first_right}")
}

fn first_value_from_left(line: &str) -> String {

    let mut value = String::from("");

    for c in line.chars() {
        if c.is_digit(10) {
            return c.to_string();
        }

        value.push(c);

        match convert_written_number(&value) {
            Some(v) => return v,
            _ => ()
        }
    }

    return "".to_string();
}

fn first_value_from_right(line: &str) -> String {

    let mut value = String::from("");

    for c in line.chars().rev() {
        if c.is_digit(10) {
            return c.to_string();
        }

        value.insert_str(0, &c.to_string());
        
        match convert_written_number(&value) {
            Some(v) => return v,
            _ => ()
        }
    }

    return String::from("");
}

fn convert_written_number(value: &str) -> Option<String>{
    let numbers = Vec::from([
        ("one", "1"),
        ("two", "2"),
        ("three", "3"),
        ("four", "4"),
        ("five", "5"),
        ("six", "6"),
        ("seven", "7"),
        ("eight", "8"),
        ("nine", "9")
    ]);

    for (k,v) in numbers {
        if value.to_string().contains(k) {
            // println!("Contains {value} <- {k}");
            return Some(v.to_string());
        }
    }

    return None;
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn should_return_a_calibration_value_of_29() {
        assert_eq!(calibration_value("two1nine"), "29".to_string());
    }

    #[test]
    fn should_return_a_calibration_value_of_83() {
        assert_eq!(calibration_value("eightwothree"), "83".to_string());
    }

    #[test]
    fn should_return_a_calibration_value_of_13() {
        assert_eq!(calibration_value("abcone2threexyz"), "13".to_string());
    }

    #[test]
    fn should_return_a_calibration_value_of_24() {
        assert_eq!(calibration_value("xtwone3four"), "24".to_string());
    }

    #[test]
    fn should_return_a_calibration_value_of_42() {
        assert_eq!(calibration_value("4nineeightseven2"), "42".to_string());
    }

    #[test]
    fn should_return_a_calibration_value_of_14() {
        assert_eq!(calibration_value("zoneight234"), "14".to_string());
    }

    #[test]
    fn should_return_a_calibration_value_of_76() {
        assert_eq!(calibration_value("7pqrstsixteen"), "76".to_string());
    }

}