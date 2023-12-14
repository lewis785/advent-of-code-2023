use std::collections::HashMap;
use regex::Regex;

fn main() {
    let input: &str = include_str!("./input.txt");
    let output: String = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let (directions, map) = parse_input(input);

    let mut step = 0;
    let directions_length = directions.len();
    let mut current_location = String::from("AAA");

    while current_location != "ZZZ" {
        let direction = directions[step % directions_length];
        current_location = next_location(&current_location, &map, &direction);
        step += 1;
    }

    step.to_string()
}

fn parse_input(input: &str)-> (Vec<char>, HashMap<&str, (String, String)>) {
    let (direction_string, map_string) = input.split_once("\n\n").unwrap();

    let directions = direction_string.chars().collect::<Vec<char>>();
    
    let mut map_hash: HashMap<&str, (String, String) > = HashMap::new();
    let location_regex = Regex::new(r"([A-Z]{3}), ([A-Z]{3})").unwrap();

    map_string.lines().for_each(|line| {
        let (key, value) = line.split_once(" = ").unwrap();
        let locations = location_regex.captures(value).unwrap();
        
        map_hash.insert(key, (locations[1].to_string(), locations[2].to_string()));
    });

    return (directions, map_hash);
}

fn next_location(current_location: &str, map: &HashMap<&str, (String, String)>, direction: &char) -> String {
    let options = &map[current_location];
    if direction == &'L' {
        return options.0.to_string()
    }
    options.1.to_string()
} 


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1("RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)");

        assert_eq!(result, "2");
    }

    #[test]
    fn test_2_part1() {
        let result = part1("LLR

AAA = (BBB, BBB)
BBB = (AAA, ZZZ)
ZZZ = (ZZZ, ZZZ)");

        assert_eq!(result, "6");
    }

    #[test]
    fn actual_part1() {
        let input = include_str!("./input.txt");
        let output = part1(input);
        assert_eq!(output, "21389")
    }
}