
struct Parts {
    map: Vec<Vec<char>>,
}

pub fn parts_sum(schema: &String) -> u32 {
    
    let parts = load_parts_map(schema);
    
    for part in parts {
        
    }
    
    return 0;
}

fn load_parts_map(schema: &String) -> Parts {
    let mut parts = Parts{map: Vec::new()};
    let lines: Vec<&str> = schema.split('\n').collect();
    for line in lines {
        let mut v: Vec<char> = Vec::new();
        for c in line.trim().chars() {
            v.push(c);
        }
        parts.map.push(v);
    }
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