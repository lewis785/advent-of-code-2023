use regex::Regex;

fn main() {
    let input: &str = include_str!("./input.txt");
    let output: String = part1(input);
    dbg!(output);
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Direction { U, L, R, D }
impl From<&str> for Direction {
    fn from(c: &str) -> Self {
        match c {
            "U" => Direction::U,
            "L" => Direction::L,
            "D" => Direction::D,
            "R" => Direction::R,
            _ => panic!("Invalid character")
        }
    }
}

enum Ground { Ditch, Flat}
impl From<char> for Ground {
    fn from(c: char) -> Self {
        match c {
            '#' => Ground::Ditch,
            '.' => Ground::Flat,
            _ => panic!("Invalid character")
        }
    }
}

#[derive(Debug)]
struct Instruction {
    direction: Direction,
    distance: u8
}

type Pos = (i32, i32);

fn parse_input(input: &str) -> Vec<Instruction> {
    let lines: Vec<&str> = input.lines().collect();
    let regex = Regex::new(r"([DLRU]{1}) (\d{1,2}) \((\#(\w){6})\)").unwrap();
    
    lines.iter().map(|line| {
        let instruct = regex.captures(line).unwrap();
        Instruction{
            direction: Direction::from(&instruct[1]),
            distance: instruct[2].parse::<u8>().unwrap_or(0)
        }
    }).collect()
}

fn part1(input: &str) -> String {
    let instructions: Vec<Instruction> = parse_input(input);
    let area = calculate_area(&instructions);
    area.to_string()
}

fn calculate_area(instructions: &Vec<Instruction>) -> i64 {
    let (perimeter, area, _) = instructions.iter().fold((0, 0, (0,0)), |(perimeter, area, pos), instruction| {
        let next_pos = next_pos(pos, instruction.direction, instruction.distance);
        let new_area: i64 = (pos.0 as i64 * next_pos.1 as i64) - (pos.1 as i64 * next_pos.0 as i64);

        (perimeter + instruction.distance as i64, area + new_area, next_pos)
    });

    ((perimeter + area as i64 ) / 2) as i64 + 1
}

fn next_pos((x, y): Pos, direction: Direction, distance: u8) -> Pos {
    match direction {
        Direction::U => (x, y - distance as i32),
        Direction::L => (x - distance as i32, y),
        Direction::D => (x, y + distance as i32),
        Direction::R => (x + distance as i32, y)     
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1("R 6 (#70c710)
D 5 (#0dc571)
L 2 (#5713f0)
D 2 (#d2c081)
R 2 (#59c680)
D 2 (#411b91)
L 5 (#8ceee2)
U 2 (#caa173)
L 1 (#1b58a2)
U 2 (#caa171)
R 2 (#7807d2)
U 3 (#a77fa3)
L 2 (#015232)
U 2 (#7a21e3)");

        assert_eq!(result, "62");
    }

    #[test]
    fn actual_part1() {
        let input = include_str!("./input.txt");
        let output = part1(input);
        assert_eq!(output, "53844")
    }
}