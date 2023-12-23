fn main() {
    let input: &str = include_str!("./input.txt");
    let output: String = part1(input);
    dbg!(output);
}

type Map = Vec<Vec<char>>;

fn part1(input: &str) -> String {
    let map = parse_input(input);
    let tilted_map = tilt_north(&map);
    let load = total_load(&tilted_map);

    load.to_string()
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    input.lines().map(|row| row.chars().collect()).collect()
}

fn tilt_north(map: &Map) -> Map {
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

        assert_eq!(result, "136");
    }

    #[test]
    fn actual_part1() {
        let input = include_str!("./input.txt");
        let output = part1(input);
        assert_eq!(output, "108889")
    }
}