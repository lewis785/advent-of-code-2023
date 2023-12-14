use std::collections::HashMap;
use regex::Regex;

fn main() {
    let input: &str = include_str!("./input.txt");
    let output: String = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    let (directions, map) = parse_input(input);

    let start: Vec<String> = start_locations(map.keys().map(|key| key.to_string()).collect());
    let destination_steps = start.iter()
    .map(|location| steps_til_finish(&map, &directions, location))
    .collect::<Vec<usize>>();
    
    destination_steps.iter()
    .fold(1, |output, &number| lowest_common_denominator(output, number))
    .to_string()
}

fn parse_input(input: &str)-> (Vec<char>, HashMap<&str, (String, String)>) {
    let (direction_string, map_string) = input.split_once("\n\n").unwrap();

    let directions = direction_string.chars().collect::<Vec<char>>();
    
    let mut map_hash: HashMap<&str, (String, String) > = HashMap::new();
    let location_regex = Regex::new(r"([A-Z1-9]{3}), ([A-Z1-9]{3})").unwrap();

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

fn start_locations(locations: Vec<String>) -> Vec<String> {
    let start_regex = Regex::new(r"A$").unwrap();
    
    locations.iter()
    .cloned()
    .filter(|location| { start_regex.is_match(location) })
    .collect()
}

fn steps_til_finish(map: &HashMap<&str, (String, String)>, directions: &Vec<char>, start_location: &str) -> usize {
    let destination_regex = Regex::new(r"Z$").unwrap();
    let directions_length = directions.len();
    let mut step = 0;
    let mut current_location = start_location.to_string();

    while !destination_regex.is_match(&current_location) {
        let direction = directions[step % directions_length];
        current_location = next_location(&current_location, &map, &direction);
        step += 1;
    }

    return step;
}

fn greatest_common_devisor(a: usize,b: usize) -> usize {
    if b == 0 {
        return a
    }
    greatest_common_devisor(b, a % b)
}

fn lowest_common_denominator(a: usize, b: usize) -> usize {
    a * b / greatest_common_devisor(a, b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let result = part2("LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)");

        assert_eq!(result, "6");
}

    #[test]
    fn actual_part2() {
        let input = include_str!("./input.txt");
        let output = part2(input);
        assert_eq!(output, "21083806112641")
    }
}