#[derive(Debug, Clone)]
struct Card {
    id: u32,
    winning: Vec<u32>,
    numbers: Vec<u32>,
    points: u32
}


pub fn winning_total(input: &String) -> u32 {
    let cards = init_cards(input);
    return 0;
}

fn init_cards(input: &String) -> Vec<Card> {

    let mut cards:Vec<Card> = Vec::new();
    let lines: Vec<&str> = input.trim().split('\n').collect();

    for (index, line) in lines.iter().enumerate() {
        let card_line: Vec<&str> = line.split('|').collect();
        let card_winning: Vec<&str> = card_line[0].split(":").collect();
        let own_numbers: Vec<&str> = card_line[1].trim().split(' ').filter(|&s| !s.is_empty()).clone().collect();

        let winning_numbers: Vec<u32> = card_winning[1].trim().split(' ').map(|v| v.parse::<u32>().unwrap()).collect();
        let own_numbers_numbers: Vec<u32> = own_numbers.iter().map(|v| v.parse::<u32>().unwrap()).collect();
        let card = Card{id: (index + 1) as u32, winning: winning_numbers, numbers: own_numbers_numbers, points: 0};
        cards.push(card);
    }

    return cards;
}

mod setup {

    pub fn load_example() -> String {
        return std::fs::read_to_string("assets/example.txt").expect("Should read test input");
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use super::setup::*;

    

    #[test]
    fn should_read_x () {
        assert_eq!(winning_total(&load_example()), 13);
    }


}