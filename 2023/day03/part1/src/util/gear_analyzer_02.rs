use std::collections::HashSet;
use std::hash::{Hash, Hasher};
use std::fs::File;
use std::io::{BufReader, BufRead};

struct ValidPosition {
    pub row: i32,
    pub col: i32,
}

#[derive(Debug, Copy, Clone)]
struct PartNumber {
    val: u32,
    pub row: i32,
    pub start: i32,
    pub end: i32
}

impl PartNumber {
    fn custom_equals(&self, other: &Self) -> bool {
        self.val == other.val
    }
}

impl Eq for PartNumber {}

impl PartialEq for PartNumber {
    fn eq(&self, other: &Self) -> bool {
        self.custom_equals(other)
    }
}

impl Hash for PartNumber {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.val.hash(state);
    }
}

type Schema = Vec<Vec<char>>;

pub fn gears_sum(reader: BufReader<File>) -> u32 {

    let mut schema: Schema = Vec::new();

    for line in reader.lines() {
        schema.push(line.unwrap().chars().collect());
    }

    let numbers = find_numbers(&schema);
    let valid_part_numbers = mark_valid_part_numbers(&numbers, &schema);

    let values: Vec<u32> = valid_part_numbers.iter().map(|v| v.val).collect();
 
    return values.iter().sum::<u32>();
}

fn find_numbers(schema: &Schema) -> Vec<PartNumber> {

    let mut parts: Vec<PartNumber> = Vec::new();

    for (row, line) in schema.iter().enumerate() {
        let mut buf: Vec<char> = Vec::new();
        for (col, c) in line.iter().enumerate() {
            if !c.is_digit(10) {
                if !buf.is_empty() {
                    let number: String = buf.iter().collect();
                    parts.push(PartNumber { val: number.parse::<u32>().unwrap(), row: row as i32, start: (col - buf.len()) as i32, end: (col - 1) as i32 });
                    buf.clear();
                }
            }
            else if c.is_digit(10) {
                buf.push(*c);
            }
        }
    }

    return parts;
}

fn mark_valid_part_numbers(part_numbers: &Vec<PartNumber>, schema: &Schema) -> HashSet<PartNumber> {
    let mut valid_part_number:HashSet<PartNumber> = HashSet::new();

    for part_number in part_numbers {
        if is_valid_part_number(part_number, schema) {
            valid_part_number.insert(part_number.clone());
        }
    }

    return valid_part_number;
}

fn is_valid_part_number(part_number: &PartNumber, schema: &Schema) -> bool {
 
    let mut valid_positions:Vec<ValidPosition>= Vec::new();
    for col in part_number.start..=part_number.end {
        valid_positions.push(ValidPosition { row: part_number.row - 1, col: col - 1});
        valid_positions.push(ValidPosition { row: part_number.row - 1, col: col});
        valid_positions.push(ValidPosition { row: part_number.row - 1, col: col + 1});
        valid_positions.push(ValidPosition { row: part_number.row, col: col - 1});
        valid_positions.push(ValidPosition { row: part_number.row, col: col + 1});
        valid_positions.push(ValidPosition { row: part_number.row + 1, col: col - 1});
        valid_positions.push(ValidPosition { row: part_number.row + 1, col: col});
        valid_positions.push(ValidPosition { row: part_number.row + 1, col: col + 1});
    }
    
    for pos in valid_positions {
        let (mx_row, mx_col) = (schema.len() - 1, schema[0].len() - 1);
        let row: i32 = if pos.row == -1 { 0 } else if pos.row > mx_row as i32 { mx_row as i32 } else { pos.row };
        let col: i32 = if pos.col == -1 { 0 } else if pos.col > mx_col as i32 { mx_col as i32 } else { pos.col };

        let schema_pos: char = schema[row as usize][col as usize];

        // It's a symbol
        if !schema_pos.is_digit(10) && schema_pos != '.' {
            return true;
        }

    }

    return false;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_calculate_correct_sum() {
        let file = File::open("assets/test_input.txt").expect("Should contain the puzzle input.");
        let reader = BufReader::new(file);

        assert_eq!(gears_sum(reader), 4361);
    }

    #[test]
    fn should_test_mod() {
        let neg:i32 = -1;
  
        assert_eq!(neg.rem_euclid(10), 0);
    }
}