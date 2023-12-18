#[derive(Debug, Clone)]
struct Card {
    id: u32,
    winning: Vec<u32>,
    numbers: Vec<u32>,
    points: u32
}

impl Card {

    fn calculate_points(&mut self) {
      let count_winners = self.winning_numbers();

        if count_winners != 0 {
            self.points = 2u32.pow(count_winners - 1);
        }
    }

    pub fn winning_numbers(&mut self) -> u32 {
      let mut count_winners = 0;

      for v in &self.numbers {
        if self.winning.contains(&v) {
            count_winners += 1;
        }
      }

      count_winners
    }

    pub fn points(mut self) -> u32 {
        if self.points == 0 {
            self.calculate_points();
        }
        return self.points;
    }
}


pub fn winning_total(input: &String) -> u32 {
    let cards = init_cards(input);
    let mut sum = 0;
    for c in cards {
        sum += c.points();
    }
    return sum;
}

pub fn scratch_card_total(input: &String) -> u32 {
  let cards: Vec<Card> = init_cards(input);


  return 0;
}


fn init_cards(input: &String) -> Vec<Card> {

    let mut cards:Vec<Card> = Vec::new();
    let lines: Vec<&str> = input.trim().split('\n').collect();

    for (index, line) in lines.iter().enumerate() {
        let card_line: Vec<&str> = line.split('|').collect();
        let card_winning: Vec<&str> = card_line[0].split(":").collect();
        let own_numbers: Vec<u32> = card_line[1].trim().split(' ').filter(|&s| !s.is_empty()).map(|v| v.parse::<u32>().unwrap()).collect();
        let winning_numbers: Vec<u32> = card_winning[1].trim().split(' ').filter(|&s| !s.is_empty()).map(|v| v.parse::<u32>().unwrap()).collect();
        let card = Card{id: (index + 1) as u32, winning: winning_numbers, numbers: own_numbers, points: 0};
        cards.push(card);
    }

    return cards;
}


#[cfg(test)]
mod tests {
    use super::*;

    pub fn load_example() -> String {
        return std::fs::read_to_string("assets/example.txt").expect("Should read test input");
    }

    #[test]
    fn should_calculate_winning () {
        assert_eq!(winning_total(&load_example()), 13);
    }

    #[test]
    fn should_calculate_total_of_scratch_cards () {
        assert_eq!(scratch_card_total(&load_example()), 30);
    }


}