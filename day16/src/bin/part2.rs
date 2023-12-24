use std::collections::{HashMap, HashSet};
use rayon::prelude::*;

fn main() {
    let input: &str = include_str!("./input.txt");
    let output: String = part2(input);
    dbg!(output);
}

#[derive(PartialEq, Eq, Hash, Clone, Copy, Debug)]
enum Direction { U, L, R, D }
type Map = Vec<Vec<char>>;
type Pos = (usize, usize);
type Step = (Pos, Direction);
type StepHistory = HashMap<String, char>;

fn parse_input(input: &str) -> Map {
    input.lines().map(|row| row.chars().collect()).collect()
}

fn part2(input: &str) -> String {
    let map: Map = parse_input(input);

    let start_squares = generate_start_steps(&map);

    let output: Vec<usize> = start_squares.par_iter().map(|step| {
        square_count(&find_energized_squares(&map, step))
    }).collect();

    output.iter().max().unwrap().to_string()
}

fn generate_start_steps(map: &Map) -> Vec<Step> {
    let mut starts: Vec<Step> = Vec::new();

    for i in 0..map.len() {
        for j in 0..map[0].len() {
            if i== 0 {
                starts.push(((j,i), Direction::D))
            }
            if j==0 {
                starts.push(((j,i), Direction::R))
            }
            if j+1 == map[0].len() {
                starts.push(((j,i), Direction::L))
            }
            if i+1 == map.len() {
                starts.push(((j,i), Direction::U))
            }         
        }
    }

    starts
}

fn find_energized_squares(map: &Map, start: &Step) -> Vec<String> {
    let mut history: StepHistory = HashMap::new();
    let mut to_visit = Vec::<Step>::new();

    to_visit.push(*start);

    while to_visit.len() > 0 {
        to_visit.iter().for_each(|step| {
            history.insert(step_to_string(step), map[step.0.1][step.0.0]);
        });

        to_visit = to_visit.iter()
        .flat_map(|step| next_step(&map, step, &history))
        .collect()
    }

    history.keys().cloned().collect()
}

fn square_count(squares: &Vec<String>) -> usize {
    let edited_squares: Vec<String> = squares.iter().map(|square| {
        square.split_once(':').unwrap().0.to_string()
    }).collect();

    HashSet::<String>::from_iter(edited_squares.iter().cloned()).len()
}

fn next_step(map: &Map, step: &Step, history: &StepHistory ) -> Vec<Step> {
    let (pos, direction) = step;
    let tile = map[pos.1][pos.0];

    let left_mirror: HashMap<Direction, Direction> = HashMap::from([
        (Direction::R, Direction::U),
        (Direction::D, Direction::L),
        (Direction::L, Direction::D),
        (Direction::U, Direction::R)
    ]); 
    let right_mirror = HashMap::from([
        (Direction::R, Direction::D),
        (Direction::D, Direction::R),
        (Direction::L, Direction::U),
        (Direction::U, Direction::L)
    ]); 
    
    let new_squares = match tile {
        '|' => { 
            if direction == &Direction::U || direction == &Direction::D { 
                vec![next_square(map, &(step.0, *direction))]
            } else { 
                vec![next_square(map, &(step.0, Direction::U)), next_square(map, &(step.0, Direction::D))]
            }
        }
        '-' => { 
            if direction == &Direction::L || direction == &Direction::R { 
                vec![next_square(map, &(step.0, *direction))]
            } else { 
                vec![next_square(map, &(step.0, Direction::L)), next_square(map, &(step.0, Direction::R))]
            }
        }
        '\\' => vec![next_square(map, &(step.0, right_mirror[direction]))],
        '/' => vec![next_square(map, &(step.0, left_mirror[direction]))],
        _ => vec![next_square(map, &(step.0, *direction))] 
    };

    let squares: Vec<Step> = new_squares.into_iter().filter_map(|option| option).collect();
    squares.iter().filter(|(pos, direction)| !history.contains_key(&step_to_string(&(*pos, *direction)))).cloned().collect()
}

fn next_square(map: &Map, (pos, direction): &Step) -> Option<Step> {
    let new_pos = match direction {
        Direction::U => if pos.1 == 0 { *pos } else { (pos.0, pos.1 - 1) },
        Direction::L => if pos.0 == 0 { *pos } else { (pos.0 - 1, pos.1) },
        Direction::D => if pos.1 + 1 >= map.len() { *pos } else { (pos.0, pos.1 + 1) },
        Direction::R => if pos.0 + 1 >= map[0].len() { *pos } else { (pos.0 + 1, pos.1) }
    };

    if &new_pos == pos {
        return None;
    }

    Some((new_pos, *direction))
}

fn step_to_string((pos, direction): &Step) -> String {
    let direction_char = match direction {
        Direction::U => 'U',
        Direction::L => 'L',
        Direction::D => 'D',
        Direction::R => 'R'
    };

    format!("{},{}:{}", pos.0, pos.1, direction_char)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let result = part2(".|...\\....
|.-.\\.....
.....|-...
........|.
..........
.........\\
..../.\\\\..
.-.-/..|..
.|....-|.\\
..//.|....");

        assert_eq!(result, "51");
    }

    #[test]
    fn actual_part2() {
        let input = include_str!("./input.txt");
        let output = part2(input);
        assert_eq!(output, "8335")
    }
}