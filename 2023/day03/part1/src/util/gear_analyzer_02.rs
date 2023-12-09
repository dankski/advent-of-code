use std::fs::File;
use std::io::{BufReader, BufRead};

struct PartNumber {
    val: u32,
    row: i32,
    start: i32,
    end: i32
}


type Schema = Vec<Vec<char>>;

pub fn gears_sum(reader: BufReader<File>) -> u32 {

    let mut schema: Schema = Vec::new();

    for line in reader.lines() {
        schema.push(line.unwrap().chars().collect());
    }

    let numbers = find_numbers(&schema);

    println!("Numbers: {}", numbers.len());
 
    return 0;
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