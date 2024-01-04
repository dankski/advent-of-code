use crate::competition::Race;

use regex::Regex;

pub fn parse_races(file: &str) -> Vec<Race> {
    let fs = std::fs::read_to_string(file).expect("Expect this files exists.");
    let lines: Vec<&str> = fs.split("\n").collect();
    let re = Regex::new(r"\d+").unwrap();
    let times: Vec<u32> = re
        .find_iter(lines[0])
        .map(|m| m.as_str().parse::<u32>().unwrap())
        .collect();
    let distandces: Vec<u32> = re
        .find_iter(lines[1])
        .map(|m| m.as_str().parse::<u32>().unwrap())
        .collect();

    return times
        .iter()
        .zip(distandces.iter())
        .map(|(t, d)| Race {
            start: *t,
            duration: *d,
        })
        .collect();
}

pub fn calculate_ways_to_win(race: &Race) -> Vec<u32> {
    let mut wins: Vec<u32> = Vec::new();

    for hold in 1..race.start {
        let diff = race.start - hold;
        let dist = diff * hold;
        if dist > race.duration {
            wins.push(hold);
        }
    }

    wins
}

pub fn error_margin(races: &Vec<Race>) -> u32 {
    let wins: Vec<u32> = races
        .iter()
        .map(|race| calculate_ways_to_win(&race).len() as u32)
        .collect::<Vec<u32>>();

    return wins.iter().fold(1, |acc, &x| acc * x);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn should_parse_example() {
        let expected_races: Vec<Race> = vec![
            Race {
                start: 7,
                duration: 9,
            },
            Race {
                start: 15,
                duration: 40,
            },
            Race {
                start: 30,
                duration: 200,
            },
        ];
        let actual_races = parse_races(&"../assets/races_example.txt");

        assert_eq!(actual_races, expected_races);
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
                start: 7,
                duration: 9,
            },
            Race {
                start: 15,
                duration: 40,
            },
            Race {
                start: 30,
                duration: 200,
            },
        ];

        assert_eq!(error_margin(&races), 288);
    }

    
}
