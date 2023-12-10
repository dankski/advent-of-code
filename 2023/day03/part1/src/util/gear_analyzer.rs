
use std::hash::{Hash, Hasher};
use std::fs::File;
use std::io::{BufReader, BufRead};
use regex::Regex;
use std::collections::HashSet;

#[derive(Debug, Eq, PartialEq, Hash)]
struct Position {
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

pub fn gears_sum(reader: BufReader<&File>) -> u32 {

    let mut schema: Schema = Vec::new();

    for line in reader.lines() {
        schema.push(line.unwrap().chars().collect());
    }

    let numbers = find_numbers(&schema);
    let valid_part_numbers = mark_valid_part_numbers(&numbers, &schema);

    let values: Vec<u32> = valid_part_numbers.iter().map(|v| v.val).collect();
 
    return values.iter().sum::<u32>();
}

pub fn gears_ratio_sum(reader: BufReader<&File>) -> u32 {
    let mut schema: Schema = Vec::new();

    for line in reader.lines() {
        schema.push(line.unwrap().chars().collect());
    }

    let numbers:Vec<PartNumber> = find_numbers(&schema);

    return find_valid_gear_numbers(&numbers, &schema);
}

fn find_numbers(schema: &Schema) -> Vec<PartNumber> {

    let mut parts: Vec<PartNumber> = Vec::new();

    let re = Regex::new(r"\d+").unwrap();

    for (row, line) in schema.iter().enumerate() {
        let string_line: String = line.iter().collect();
        let part_nums: Vec<PartNumber> = re
                                        .find_iter(&string_line)
                                        .map(|m| PartNumber{val: m.as_str().parse::<u32>().unwrap(), row: row as i32, start: m.start() as i32, end: m.end() as i32})
                                        .collect();
        part_nums.iter().for_each(|pn| parts.push(pn.clone()));

    }

    return parts;
}

fn mark_valid_part_numbers(part_numbers: &Vec<PartNumber>, schema: &Schema) -> Vec<PartNumber> {
    let mut valid_part_number:Vec<PartNumber> = Vec::new();

    for part_number in part_numbers {
        if is_valid_part_number(part_number, schema) {
            valid_part_number.push(part_number.clone());
        }
    }

    return valid_part_number;
}

fn find_valid_gear_numbers(part_numbers: &Vec<PartNumber>, schema: &Schema) -> u32 {
   let mut valid_gears:Vec<u32> = Vec::new();
   let mut valid_numbers: Vec<PartNumber> = part_numbers.clone();

   for (row_index, row) in schema.iter().enumerate() {
        for (col_index, elem) in row.iter().enumerate() {
            let mut valid_positions:HashSet<Position> = HashSet::new();
            if is_valid_gear(*elem) {
                valid_positions.insert(Position { row: row_index as i32 - 1, col: col_index as i32 - 1});
                valid_positions.insert(Position { row: row_index as i32 - 1, col: col_index as i32});
                valid_positions.insert(Position { row: row_index as i32 - 1, col: col_index as i32 + 1});
                valid_positions.insert(Position { row: row_index as i32, col: col_index as i32 - 1});
                valid_positions.insert(Position { row: row_index as i32, col: col_index as i32 + 1});
                valid_positions.insert(Position { row: row_index as i32 + 1, col: col_index as i32 - 1});
                valid_positions.insert(Position { row: row_index as i32 + 1, col: col_index as i32});
                valid_positions.insert(Position { row: row_index as i32 + 1, col: col_index as i32 + 1});

                let mut candidates: Vec<PartNumber> = Vec::new();
                for part_number in &valid_numbers {
                    if is_at_valid_location(part_number, &valid_positions) {
                        candidates.push(part_number.clone());
                    }
                }
                
                if candidates.len() == 2 {
                    valid_gears.push(candidates.iter().map(|p| p.val).reduce(|a, b| a * b).unwrap());
                    // Subtract candidates
                    valid_numbers.retain(|&part| !&candidates.contains(&part));
                }
            }
        }
   }

   return valid_gears.iter().sum();

}

fn is_at_valid_location(part_number: &PartNumber, valid_positions: &HashSet<Position>) -> bool {
    let mapped_part_numbers: HashSet<Position> = (part_number.start..part_number.end).into_iter().map(|i| Position{row: part_number.row, col: i}).collect();
    let matches = mapped_part_numbers.intersection(valid_positions);

    return matches.count() != 0;
}

fn is_valid_part_number(part_number: &PartNumber, schema: &Schema) -> bool {
 
    let mut valid_positions:Vec<Position>= Vec::new();
    for col in part_number.start..part_number.end {
        valid_positions.push(Position { row: part_number.row - 1, col: col - 1});
        valid_positions.push(Position { row: part_number.row - 1, col: col});
        valid_positions.push(Position { row: part_number.row - 1, col: col + 1});
        valid_positions.push(Position { row: part_number.row, col: col - 1});
        valid_positions.push(Position { row: part_number.row, col: col + 1});
        valid_positions.push(Position { row: part_number.row + 1, col: col - 1});
        valid_positions.push(Position { row: part_number.row + 1, col: col});
        valid_positions.push(Position { row: part_number.row + 1, col: col + 1});
    }
    
    let mut match_postions: Vec<bool> = Vec::new();
    for pos in valid_positions {
        let (mx_row, mx_col) = (schema.len() - 1, schema[0].len() - 1);
        let row: i32 = if pos.row < 0 { 0 } else if pos.row > mx_row as i32 { mx_row as i32 } else { pos.row };
        let col: i32 = if pos.col < 0 { 0 } else if pos.col > mx_col as i32 { mx_col as i32 } else { pos.col };

        let schema_char: char = schema[row as usize][col as usize];

        if is_valid_part_number_location_designator(schema_char) {
            match_postions.push(true);
        }
    }

    return match_postions.iter().find(|&&b| b).is_some();
}

fn is_valid_part_number_location_designator(c: char) -> bool {
    !c.is_digit(10) && c != '.'
}

fn is_valid_gear(c: char) -> bool {
    c == '*'
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_calculate_correct_sum() {
        let file = File::open("assets/test_input_00.txt").expect("Should contain the puzzle input.");
        let reader = BufReader::new(&file);

        assert_eq!(gears_sum(reader), 4361);
    }

    #[test]
    fn should_calculate_small_sum_01() {
        let file = File::open("assets/test_input_01.txt").expect("Should contain the puzzle input.");
        let reader = BufReader::new(&file);

        assert_eq!(gears_sum(reader), 1209);
    }

    #[test]
    fn should_calculate_small_sum_02() {
        let file = File::open("assets/test_input_02.txt").expect("Should contain the puzzle input.");
        let reader = BufReader::new(&file);

        assert_eq!(gears_sum(reader), 675);
    }

    #[test]
    fn should_calculate_small_sum_03() {
        let file = File::open("assets/test_input_03.txt").expect("Should contain the puzzle input.");
        let reader = BufReader::new(&file);

        assert_eq!(gears_sum(reader), 862);
    }

    #[test]
    fn should_calculate_gear_sum_00() {
        let file = File::open("assets/test_input_00.txt").expect("Should contain the puzzle input.");
        let reader = BufReader::new(&file);

        assert_eq!(gears_ratio_sum(reader), 467835);
    }

}