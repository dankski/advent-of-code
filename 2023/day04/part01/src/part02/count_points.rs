
use regex::Regex;

struct Game {
    game_number: u32,
    winning_numbers: Vec<u32>,
    actual_numbers: Vec<u32>
}

pub fn calculate_total_scratch_cards(lines: Vec<&str>) -> u32 {
    let games: Vec<Game> = lines.iter().map(|l| parse_games(l)).collect();

    return 0;
}

fn parse_games(line: &str) -> Game {
    let re = Regex::new(r"\d+").unwrap();
    let a = line
        .split_once(':')
        .unwrap().0.chars().partition(|&s| s.is_digit(10)).0.as_str();
    
    Game{game_number: 9, winning_numbers: Vec::new(), actual_numbers: Vec::new()}
}



#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_calculate_total_scratch_cards() {
        let input = std::fs::read_to_string("assets/input.txt").expect("Should read test input");
        let lines: Vec<&str> = input.trim().split('\n').collect();

        assert_eq!(calculate_total_scratch_cards(lines), 30);
    }
}