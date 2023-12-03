

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

    // (y,x)
    let north: (i32, i32) = (-1, 0);
    let south: (i32, i32) = (1, 0);
    let west: (i32, i32) = (0, -1);
    let east: (i32, i32) = (0, 1);

    let north_west: (i32, i32) = (-1, -1);
    let north_east: (i32, i32) = (-1, 1);
    let south_west: (i32, i32) = (1, -1);
    let south_east: (i32, i32) = (1, 1);

    for number in numbers {
        for x in number.x as i32 .. (number.x + number.val.len() as i32) {
            let current_pos = (number.y, x);
            let n_pos = add_pos(current_pos, north, &parts);
            let s_pos = add_pos(current_pos, south, &parts);
            let w_pos = add_pos(current_pos, west, &parts);
            let e_pos = add_pos(current_pos, east, &parts);

            let nw_pos = add_pos(current_pos, north_west, &parts);
            let ne_pos = add_pos(current_pos, north_east, &parts);
            let sw_pos = add_pos(current_pos, south_west, &parts);
            let se_pos = add_pos(current_pos, south_east, &parts);

            if has_adjacent(n_pos, parts) 
                || has_adjacent(s_pos, parts) 
                || has_adjacent(w_pos, parts) 
                || has_adjacent(e_pos, parts) 
                || has_adjacent(nw_pos, parts) 
                || has_adjacent(ne_pos, parts) 
                || has_adjacent(sw_pos, parts) 
                || has_adjacent(se_pos, parts) {

                found.push(Number{val: String::from(&number.val), x: number.x, y: number.y});
                break;
            }
  
        }
    }

    return found;
}

fn add_pos(a: (i32, i32), b: (i32, i32), parts: &Parts) -> (u32, u32) {
    let mut pos =(a.0 + b.0, a.1 + b.1);
    if pos.0 < 0 || pos.0 > parts.y_dim - 1 {
        pos.0 = 0;
    }

    if pos.1 < 0 || pos.1 > parts.x_dim - 1 {
        pos.1 = 0;
    }

    return (pos.0 as u32, pos.1 as u32);
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