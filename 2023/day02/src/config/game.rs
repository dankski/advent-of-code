
#[derive(Copy, Clone)]
pub struct Game {
    load: BagLoading,
}

#[derive(Copy, Clone)]
struct BagLoading {
    blue: u32,
    green: u32,
    red: u32
}

impl Game {

    pub fn new() -> Self {
        Game{load: BagLoading{red: 12, blue: 14, green: 13}}
    }

    pub fn possible_game(self, g: &String) -> (u32, u32) {
        let values: Vec<&str> = g.split(|c| c == ':' || c == ';').collect();
        
        let gid = self.extract_game_id(values[0]);
        let mut min_set:(u32,u32,u32) = (0,0,0);
        for i in 1..values.len() {
            let r = values[i];
            let validation_result = self.valdiate_round(r, min_set);
            if validation_result.0 != true {
                return (0, 0);
            }
            min_set = validation_result.1;
        }

        println!("\n min set: rgb({}, {}, {}) \n", min_set.0, min_set.1, min_set.2);
        return (gid, min_set.0 * min_set.1 * min_set.2);
    }

    fn extract_game_id(self, s: &str) -> u32 {
        let v: Vec<&str> = s.split("Game ").collect();
        return v[1].parse::<u32>().unwrap();
    }

    fn valdiate_round(self, round: &str, min_set: (u32, u32, u32)) -> (bool, (u32, u32, u32)) {
        let colors: Vec<String> = round.split(',').map(|e| e.to_string()).collect();
 
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        let mut min_set_result = min_set;
        for color in colors {

            if color.contains("red") {
                let result: Vec<&str> = color.trim().split(char::is_whitespace).collect();
                red = result[0].parse::<u32>().unwrap();
            }

            if color.contains("blue") {
                let result: Vec<&str> = color.trim().split(char::is_whitespace).collect();
                blue = result[0].parse::<u32>().unwrap();
            }

            if color.contains("green") {
                let result: Vec<&str> = color.trim().split(char::is_whitespace).collect();
                green = result[0].parse::<u32>().unwrap();
            }

            if !self.validate((red, green, blue)) {
                return (false, (0,0,0));
            }

            min_set_result = self.minimum_cube_set(min_set_result, (red, green, blue));
        }


        return (true, min_set_result);
    }

    fn validate(self, round: (u32, u32, u32)) -> bool {

        if self.load.red >= round.0 && self.load.green >= round.1 && self.load.blue >= round.2 {
            return true;
        }
        return false;
    }

    fn minimum_cube_set(self, min_cube_set: (u32, u32, u32), current_set: (u32, u32, u32)) -> (u32, u32, u32) {
        let mut min_result: (u32, u32, u32) = min_cube_set;

        if current_set.0 >= min_cube_set.0 {
            min_result.0 = current_set.0;
        }

        if current_set.1 >= min_cube_set.1 {
            min_result.1 = current_set.1;
        }

        if current_set.2 >= min_cube_set.2 {
            min_result.2 = current_set.2;
        }
        
        return min_result;
    }
    
}

#[cfg(test)]
mod tests {

    use super::*;
    
    #[test]
    fn should_return_game1_ok() {
        let g = Game::new();
        
        assert_eq!(g.possible_game(&"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string()), (1, 48));
    }

    #[test]
    fn should_return_game2_ok() {
        let g = Game::new();

        assert_eq!(g.possible_game(&"Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue".to_string()), (2, 12));
    }

    #[test]
    fn should_return_game3_not_ok() {
        let g = Game::new();

        assert_eq!(g.possible_game(&"Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red".to_string()), (0, 0));
    }


    #[test]
    fn should_return_game4_not_ok() {
        let g = Game::new();

        assert_eq!(g.possible_game(&"Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red".to_string()), (0, 0));
    }

    #[test]
    fn should_return_game5_ok() {
        let g = Game::new();

        assert_eq!(g.possible_game(&"Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green".to_string()), (5, 0));
    }

}