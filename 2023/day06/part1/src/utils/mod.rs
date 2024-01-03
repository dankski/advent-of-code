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
}
