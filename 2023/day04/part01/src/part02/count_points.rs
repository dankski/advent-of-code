
use regex::Regex;

#[derive(Debug, Clone)]
struct Game {
    game_number: u32,
    winning_numbers: Vec<u32>,
    actual_numbers: Vec<u32>
}

pub fn calculate_total_scratch_cards(lines: Vec<&str>) -> u32 {

    let games: Vec<Game> = lines.iter().map(|l| parse_games(l)).collect();

    let mut result = games.len();

    let mut game_copies: Vec<Game> = games.iter().flat_map(|i| copies(i, &games)).collect();

    while !game_copies.is_empty() {
        result = result + game_copies.len();
        game_copies = game_copies.iter_mut().flat_map(|game| copies(game, &games)).collect();
    }

    return result as u32; 
}

fn parse_games(line: &str) -> Game {

    let re = Regex::new(r"\d+").unwrap();
    let collon_split: Vec<&str> = line.split(':').collect();
    let line_before = collon_split[0];
    let line_after  = collon_split[1];
    let game_number: Vec<_> = re.find_iter(line_before).map(|m| m.as_str().parse::<u32>()).collect();
    let numbers: Vec<&str> = line_after.split('|').map(|s| s.trim()).collect();
    let winning_numbers: Vec<u32> = re.find_iter(numbers[0]).map(|m| m.as_str().parse::<u32>().unwrap()).collect();
    let actual_numbers: Vec<u32> = re.find_iter(numbers[1]).map(|m| m.as_str().parse::<u32>().unwrap()).collect();
    
    Game{game_number: game_number[0].clone().unwrap(), winning_numbers: winning_numbers.clone(), actual_numbers: actual_numbers.clone()}
}

fn copies(game: &Game, games: &Vec<Game>) -> Vec<Game> {
    let hits = count_hits(game);
    if hits < 1 {
       return Vec::new();
    } else {
        return games[game.game_number as usize..(game.game_number + hits) as usize].to_vec();
    }
}

fn count_hits(game: &Game) -> u32 {
    let hits: Vec<u32> = game.actual_numbers.iter().filter(|&n| game.winning_numbers.contains(n)).cloned().collect();
    hits.len() as u32
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_calculate_total_scratch_cards_from_the_example() {
        let input = std::fs::read_to_string("assets/example.txt").expect("Should read test input");
        let lines: Vec<&str> = input.trim().split('\n').collect();

        assert_eq!(calculate_total_scratch_cards(lines), 30);
    }

    #[test]
    fn should_calculate_total_scratch_cards() {
        let input = std::fs::read_to_string("assets/input.txt").expect("Should read test input");
        let lines: Vec<&str> = input.trim().split('\n').collect();

        assert_eq!(calculate_total_scratch_cards(lines), 30);
    }
}