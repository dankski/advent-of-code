use crate::competition::Race;

pub fn parse_race(file: &str) -> Vec<Race> {
    let fs = std::fs::read_to_string(file).expect("Expect this files exists.");
    let lines: Vec<&str> = fs.trim().split("\n").collect();

    let numbers = lines
        .iter()
        .map(|line| line.trim())
        .map(|line| {
            line.replace(" ", "")
                .split(":")
                .collect::<Vec<&str>>()
                .last()
                .unwrap()
                .to_string()
        })
        .collect::<Vec<String>>();

    vec![Race {
        start: numbers[0].parse::<u64>().unwrap(),
        duration: numbers[1].parse::<u64>().unwrap(),
    }]
}

pub fn error_margin(races: &Vec<Race>) -> u64 {
    let wins: Vec<u64> = races
        .iter()
        .map(|race| calculate_ways_to_win(&race).len() as u64)
        .collect::<Vec<u64>>();

    return wins.iter().fold(1, |acc, &x| acc * x);
}

pub fn calculate_ways_to_win(race: &Race) -> Vec<u64> {
    let mut wins: Vec<u64> = Vec::new();

    for hold in 1..race.start {
        let diff = race.start - hold;
        let dist = diff * hold;
        if dist > race.duration {
            wins.push(hold);
        }
    }

    wins
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_example() {
        let expected_race = vec![Race {
            start: 71530,
            duration: 940200,
        }];

        let actual_races = parse_race(&"../assets/races_example.txt");

        assert_eq!(actual_races, expected_race);
    }

    #[test]
    fn should_calculate_four_ways() {
        let race = Race {
            start: 7,
            duration: 9,
        };

        assert_eq!(calculate_ways_to_win(&race), vec![2, 3, 4, 5]);
    }

    #[test]
    fn should_calculate_eight_ways() {
        let race = Race {
            start: 15,
            duration: 40,
        };

        assert_eq!(calculate_ways_to_win(&race), vec![4, 5, 6, 7, 8, 9, 10, 11]);
    }

    #[test]
    fn should_calculate_nine_ways() {
        let race = Race {
            start: 30,
            duration: 200,
        };

        assert_eq!(
            calculate_ways_to_win(&race),
            vec![11, 12, 13, 14, 15, 16, 17, 18, 19]
        );
    }

    #[test]
    fn should_calculate_error_margin() {
        let races: Vec<Race> = vec![
            Race {
                start: 71530,
                duration: 940200,
            }
        ];

        assert_eq!(error_margin(&races), 71503);
    }
}
