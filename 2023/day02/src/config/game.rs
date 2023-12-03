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

    pub fn possible_game(self, g: &String) -> u32 {
        let values: Vec<&str> = g.split(|c| c == ':' || c == ';').collect();
        // values.into_iter().for_each(|e| println!("{e}"));
        let gid = self.extract_game_id(values[0]);
        for i in 1..values.len() {
            let r = values[i];
            if self.valdiate_round(r) != true {
                return 0;
            }
        }

        return 0;
    }

    fn extract_game_id(self, s: &str) -> u32 {
        let v: Vec<&str> = s.split("Game ").collect();
        return v[1].parse::<u32>().unwrap();
    }

    fn valdiate_round(self, round: &str) -> bool {

        let colors: Vec<String> = round.split(',').map(|e| e.to_string()).collect();

        let red = 0;
        let green = 0;
        let blue = 0;

        // for color in colors {
        //     println!("{}", color);
        //     red: u32 = self.read_color_value(&"red", &color);       
        //     green = self.read_color_value(&"green", &color);
        //     let blue = self.read_color_value(&"blue", &color);
    
        //     println!("{red} {green} {blue}");  
        // }

        return true;
    }

    fn read_color_value(self,color: &str, s: &String) -> u32 {
        if s.contains(color) {
            let result: Vec<&str> = s.trim().split(char::is_whitespace).collect();
            return result[0].parse::<u32>().unwrap();
        }
        return 0;
    }
    
}

#[cfg(test)]
mod tests {

    use super::*;
    
    #[test]
    fn should_return_game1_ok() {
        let g = Game::new();

        assert_eq!(g.possible_game(&"Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green".to_string()), 1);
    }



}