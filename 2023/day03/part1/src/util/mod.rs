use core::num;
use std::{ptr::null, sync::Arc};


struct Parts {
    map: Vec<Vec<char>>,
    x_dim: i32,
    y_dim: i32
}

struct Number {
    pub val: String,
    pub x: i32,
    pub y: i32
}

impl Parts {
    fn init(&mut self) {
        self.x_dim = self.map[0].len() as i32;
        self.y_dim = self.map.len() as i32;
    }

    fn possible_positions(self, pos: u32) -> Vec<(u32,u32)> {
        return Vec::new();
    }
}
pub fn parts_sum(schema: &String) -> u32 {
    
    let parts = load_parts_map(schema);
    let numbers = find_numbers(&parts);
    let filtered_numbers = filter_numbers(&numbers, &parts);
    
    let mut sum = 0;
    for n in filtered_numbers {
        sum = sum + n.val.parse::<u32>().unwrap();
    }
    
    return sum;
}

fn filter_numbers(numbers: &Vec<Number>, parts: &Parts) -> Vec<Number> {
    let mut found = Vec::new();

    for number in numbers {
        for x in number.x as i32 .. (number.x + number.val.len() as i32) {
            let north: (u32, u32) = if number.y > 0 { ((number.y - 1) as u32, x as u32)} else{(0 as u32, 0 as u32)};
            let south: (u32, u32) = if number.y < parts.y_dim - 1 { ((number.y + 1) as u32, x as u32)} else {(0 as u32, 0 as u32)};
            let west: (u32, u32) = if x > 0 { (number.y as u32, (x - 1) as u32)} else {(0 as u32, 0 as u32)};
            let east: (u32, u32) = if x < parts.x_dim - 1 { (number.y as u32, (x + 1) as u32) } else {(0 as u32, 0 as u32)};

            let north_west: (u32, u32) = if number.y > 0 && x > 0 { ((number.y - 1) as u32, (x - 1) as u32)} else{(0 as u32, 0 as u32)};
            let north_east: (u32, u32) = if number.y > 0 && x < parts.x_dim - 1 { ((number.y - 1) as u32, (x + 1) as u32)} else{(0 as u32, 0 as u32)};
            let south_west: (u32, u32) = if number.y < parts.y_dim - 1 && x > 0 {((number.y + 1) as u32, (x - 1) as u32)} else{(0 as u32, 0 as u32)};
            let south_east: (u32, u32) = if number.y < parts.y_dim - 1 && x < parts.x_dim - 1 {((number.y + 1) as u32, (x + 1) as u32)} else{(0 as u32, 0 as u32)};

            if has_adjacent(north, parts) 
                || has_adjacent(south, parts) 
                || has_adjacent(west, parts) 
                || has_adjacent(east, parts) 
                || has_adjacent(north_west, parts) 
                || has_adjacent(north_east, parts) 
                || has_adjacent(south_west, parts) 
                || has_adjacent(south_east, parts) {

                found.push(Number{val: String::from(&number.val), x: number.x, y: number.y});
                break;
            }
  
        }
    }

    return found;
}

fn has_adjacent(pos: (u32, u32), parts: &Parts) -> bool {
    let row = parts.map.get(pos.0 as usize).unwrap();
    let c = row.get(pos.1 as usize).unwrap();
    if *c != '.' && !c.is_alphanumeric() {
        return true;
    }
    return false;
}

fn find_numbers(parts: &Parts) -> Vec<Number> {
    let mut numbers: Vec<Number> = Vec::new();
    let mut start_reading_num: bool = false;
    for (i, part) in parts.map.iter().enumerate() {
        let mut number = Number{val: String::from(""), x:0, y:0};
        for (j, c) in part.iter().enumerate() {

            if c.is_digit(10) {
                if start_reading_num == false {
                    start_reading_num = true;
                    number.val.push(*c);
                    number.y = i as i32;
                    number.x = j as i32;
                }
                else {
                    number.val.push(*c);
                }
            }
            else {
                if start_reading_num == true {
                    start_reading_num = false;
                    numbers.push(Number{val: number.val, x: number.x, y: number.y});
                    number.val = String::from("");
                    number.x = 0;
                    number.y = 0;
                }
            }
        }
    } 
    return numbers;
}

fn load_parts_map(schema: &String) -> Parts {
    let mut parts = Parts{map: Vec::new(), x_dim: 0, y_dim: 0};
    let lines: Vec<&str> = schema.split('\n').collect();
    for line in lines {
        let mut v: Vec<char> = Vec::new();
        for c in line.trim().chars() {
            v.push(c);
        }
        parts.map.push(v);
    }
    parts.init();
    return parts;
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_calculate_parts() {
        let schema = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..".to_string();

        assert_eq!(parts_sum(&schema), 4361);
    }
}