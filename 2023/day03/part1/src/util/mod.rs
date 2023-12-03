
struct Parts {
    map: Vec<Vec<char>>,
    x_dim: u32,
    y_dim: u32
}

impl Parts {
    fn init(&mut self) {
        self.x_dim = self.map[0].len() as u32;
        self.y_dim = self.map.len() as u32;
    }

    fn posible_positions(self, pos: u32) -> Vec<(u32,u32)> {
        
    }
}
pub fn parts_sum(schema: &String) -> u32 {
    
    let parts = load_parts_map(schema);
    
    for part in parts.map {
        for c in part.chars() {
            
        }
    }   
    
    return 0;
}

fn load_parts_map(schema: &String) -> Parts {
    let mut parts = Parts{map: Vec::new(), x_dim: 0, y_dim: 0};
    let lines: Vec<&str> = schema.split('\n').collect();
    for line in lines {
        let mut v: Vec<char> = Vec::new();
        for (j, c) in line.trim().chars().enumerate() {
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