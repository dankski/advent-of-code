mod model;

use std::{ops::Add, vec};

use model::CategoryMap;
use regex::Regex;

use self::model::Line;

struct Seeds {
    values: Vec<u64>,
}

struct Almanac {
    lines: Vec<String>,
}

pub fn lowest_location(almanac: &String) -> u64 {
    let almanac = load_almanac(almanac);

    let seeds = parse_seeds(&almanac.lines[0]);

    let seed_to_soil = parse_cat_map(&almanac, "seed-to-soil");
    let soil_to_fertilzer = parse_cat_map(&almanac, "soil-to-fertilizer");
    let fertilzer_to_water = parse_cat_map(&almanac, "fertilizer-to-water");
    let water_to_light = parse_cat_map(&almanac, "water-to-light");
    let light_to_temp = parse_cat_map(&almanac, "light-to-temperature");
    let temp_to_hum = parse_cat_map(&almanac, "temperature-to-humidity");
    let hum_to_loc = parse_cat_map(&almanac, "humidity-to-location");

    let maps = vec![
        seed_to_soil,
        soil_to_fertilzer,
        fertilzer_to_water,
        water_to_light,
        light_to_temp,
        temp_to_hum,
        hum_to_loc,
    ];

    return calculate_lowest_location(&seeds.values, &maps);
}

pub fn lowest_location_part_2(almanac: &String) -> u64 {
    let almanac = load_almanac(almanac);

    let seeds = parse_seeds(&almanac.lines[0]);

    let seed_to_soil = parse_cat_map(&almanac, "seed-to-soil");
    let soil_to_fertilzer = parse_cat_map(&almanac, "soil-to-fertilizer");
    let fertilzer_to_water = parse_cat_map(&almanac, "fertilizer-to-water");
    let water_to_light = parse_cat_map(&almanac, "water-to-light");
    let light_to_temp = parse_cat_map(&almanac, "light-to-temperature");
    let temp_to_hum = parse_cat_map(&almanac, "temperature-to-humidity");
    let hum_to_loc = parse_cat_map(&almanac, "humidity-to-location");

    let maps = vec![
        seed_to_soil,
        soil_to_fertilzer,
        fertilzer_to_water,
        water_to_light,
        light_to_temp,
        temp_to_hum,
        hum_to_loc,
    ];

    return calculate_lowest_location_part_2(&seeds.values, &maps);
}

fn load_almanac(almanac: &String) -> Almanac {
    Almanac {
        lines: almanac.lines().map(|l| l.to_owned()).collect(),
    }
}

fn parse_seeds(almanac: &String) -> Seeds {
    let re = Regex::new(r"\d+").unwrap();

    Seeds {
        values: almanac
            .lines()
            .filter(|l| l.trim().contains("seeds:"))
            .map(|l| l.split("seeds: ").last().unwrap_or(""))
            .map(|l| re.find_iter(l).map(|m| m.as_str().parse::<u64>()))
            .flat_map(|v| v.map(|n| n.unwrap()))
            .collect(),
    }
}

fn parse_cat_map(almanac: &Almanac, map_name: &str) -> CategoryMap {
    let re = Regex::new(r"\d+").unwrap();
    let mut soil_map: Vec<Line> = Vec::new();

    let mut start: usize = 0;
    let name_filter: String = map_name.to_string().add(" map:");
    for (index, line) in almanac.lines.iter().enumerate() {
        if line.contains(&name_filter) {
            start = index + 1;
            break;
        }
    }

    for line in &almanac.lines[start..] {
        if !line.eq("") {
            let numbers: Vec<u64> = re
                .find_iter(line)
                .map(|m| m.as_str().parse::<u64>().unwrap())
                .collect();
            soil_map.push(Line {
                dst: numbers[0],
                src: numbers[1],
                range: numbers[2],
            })
        } else {
            break;
        }
    }

    CategoryMap {
        name: map_name.to_string(),
        lines: soil_map.clone(),
    }
}

fn calculate_lowest_location_part_2(seeds: &Vec<u64>, maps: &Vec<CategoryMap>) -> u64 {
    let seed_pairs: Vec<Vec<u64>> = seeds.chunks(2).map(|chunk| chunk.to_vec()).collect();

    let seed_range: Vec<u64> = seed_pairs
        .iter()
        .flat_map(|sp| (sp[0]..sp[0] + sp[1]).collect::<Vec<u64>>())
        .collect();

    return calculate_lowest_location(&seed_range, &maps);
}

fn calculate_lowest_location(seeds: &Vec<u64>, maps: &Vec<CategoryMap>) -> u64 {
    let mut result = u64::MAX;

    seeds.iter().for_each(|seed| {
        let mut final_seed = *seed;

        maps.iter()
            .for_each(|m| final_seed = propagate_seed(final_seed, &m));

        result = result.min(final_seed);
    });

    return result;
}

fn propagate_seed(seed: u64, cat_map: &CategoryMap) -> u64 {
    for line in &cat_map.lines {
        if seed >= line.src && seed < (line.src + line.range) {
            return line.dst + (seed - line.src);
        }
    }
    return seed;
}

#[cfg(test)]
mod tests {
    use std::vec;

    use super::*;

    #[test]
    fn should_return_lowest_location_for_the_whole_shebang() {
        let mini_almanac =
            std::fs::read_to_string("assets/input.txt").expect("This file should exist");

        assert_eq!(lowest_location(&mini_almanac), 525792406);
    }

    #[test]
    fn should_return_lowest_location() {
        let mini_almanac =
            std::fs::read_to_string("assets/example.txt").expect("This file should exist");

        assert_eq!(lowest_location(&mini_almanac), 35);
    }

    #[test]
    fn should_return_lowest_location_part_02() {
        let mini_almanac =
            std::fs::read_to_string("assets/input.txt").expect("This file should exist");

        assert_eq!(lowest_location_part_2(&mini_almanac), 79004094);
    }

    #[test]
    fn should_return_lowest_location_example_part_02() {
        let mini_almanac =
            std::fs::read_to_string("assets/example.txt").expect("This file should exist");

        assert_eq!(lowest_location_part_2(&mini_almanac), 46);
    }

    #[test]
    fn should_parse_seeds() {
        let text = "Test\nseeds: 79 14 55 13\nWtf\n";
        let re = Regex::new(r"\d+").unwrap();
        let result: Vec<u64> = text
            .lines()
            .filter(|l| l.trim().contains("seeds:"))
            .map(|l| l.split("seeds: ").last().unwrap_or(""))
            .map(|l| re.find_iter(l).map(|m| m.as_str().parse::<u64>()))
            .flat_map(|v| v.map(|n| n.unwrap()))
            .collect();

        assert_eq!(result, vec![79, 14, 55, 13]);
    }

    #[test]
    fn should_generate_seed_to_soil_map() {
        let cat_map: CategoryMap = CategoryMap {
            name: "seed to soil".to_string(),
            lines: vec![
                Line {
                    dst: 50,
                    src: 98,
                    range: 2,
                },
                Line {
                    dst: 52,
                    src: 50,
                    range: 48,
                },
            ],
        };

        let result = cat_map.generate_mapping();
        let expected: Vec<u64> = vec![
            0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23,
            24, 25, 26, 27, 28, 29, 30, 31, 32, 33, 34, 35, 36, 37, 38, 39, 40, 41, 42, 43, 44, 45,
            46, 47, 48, 49, 52, 53, 54, 55, 56, 57, 58, 59, 60, 61, 62, 63, 64, 65, 66, 67, 68, 69,
            70, 71, 72, 73, 74, 75, 76, 77, 78, 79, 80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 91,
            92, 93, 94, 95, 96, 97, 98, 99, 50, 51,
        ];

        assert_eq!(result, expected);
    }
}
