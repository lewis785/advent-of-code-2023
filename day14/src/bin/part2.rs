use std::collections::HashMap;

fn main() {
    let input: &str = include_str!("./input.txt");
    let output: String = part1(input);
    dbg!(output);
}

type Map = Vec<Vec<char>>;

fn part1(input: &str) -> String {
    let map = parse_input(input);
    
    let (loop_start, loop_count) = find_loop(&map);
    let cycles = loop_start + ((1000000000 - loop_start) % loop_count);
    let mut result_map = map;
    
    for _ in 0..cycles {
        result_map = cycle(&result_map);
    }
    
    let load = total_load(&result_map);
    load.to_string()
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|row| row.chars().collect()).collect()
}

fn tilt(map: &Map) -> Map {
    let mut new_map: Map = vec![vec!['.';map[0].len()]; map.len()];
    let mut free_spots = vec![0; map[0].len()];

    for (row_index, row) in map.iter().enumerate() {
        for (column_index, column) in row.iter().enumerate() {
            if column == &'#' {
                free_spots[column_index] = row_index + 1;
                new_map[row_index][column_index] = '#'
            }

            if column == &'O' {
                new_map[free_spots[column_index]][column_index] = 'O';
                free_spots[column_index] += 1;
            }
        }
    }

    new_map
}

fn total_load(map: &Map) -> u32 {
    let height = map.len();

    map.iter().enumerate().map(|(index, row)| {
        row.iter().map(|column| {
            if column == &'O' { (height - index) as u32 } else { 0 }
        }).sum::<u32>()
    }).sum::<u32>()
}

fn rotate_map(puzzle: &Map) -> Map {
    let columns = puzzle[0].len();
    let rows = puzzle.len();
    let mut rotated_map: Map = vec![vec!['X'; rows]; columns];

    for i in 0..rows {
        for j in 0..columns {
            rotated_map[j][rows - 1 - i] = puzzle[i][j]
        }
    }

    rotated_map
}

fn cycle(map: &Map) -> Map {
    let north_tilt = tilt(map);
    let west_tilt = tilt(&rotate_map(&north_tilt));
    let south_tilt = tilt(&rotate_map(&west_tilt));
    let east_tilt = tilt(&rotate_map(&south_tilt));
    rotate_map(&east_tilt)
}

fn find_loop(map: &Map) -> (usize, usize) {
    let mut map_hash = HashMap::<String, usize>::new();
    let mut next_map = map.clone();
    let mut cycle_count = 0;
    

    let mut map_string = map.iter()
    .map(|row| row.iter().collect::<String>())
    .collect::<String>();


    while !map_hash.contains_key(&map_string) {
        map_hash.insert(map_string, cycle_count);

        next_map = cycle(&next_map);

        map_string = next_map.iter()
        .map(|row| row.iter().collect::<String>())
        .collect::<String>();
        cycle_count += 1;
    }
    
    (map_hash[&map_string], cycle_count - map_hash[&map_string])
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1("O....#....
O.OO#....#
.....##...
OO.#O....O
.O.....O#.
O.#..O.#.#
..O..#O..O
.......O..
#....###..
#OO..#....");

        assert_eq!(result, "64");
    }

    #[test]
    fn actual_part1() {
        let input = include_str!("./input.txt");
        let output = part1(input);
        assert_eq!(output, "104671")
    }
}