fn main() {
    let input: &str = include_str!("./input.txt");
    let output: String = part1(input);
    dbg!(output);
}

type Puzzle = Vec<Vec<char>>;

fn part1(input: &str) -> String {
    let puzzles = parse_input(input);

    let output: u32 = puzzles.iter()
    .map(|puzzle| mirror_value(puzzle))
    .sum();
    
    output.to_string()
}

fn parse_input(input: &str) -> Vec<Puzzle> {
    let puzzles: Vec<&str> = input.split("\n\n").collect();

    puzzles.iter()
    .map(|puzzle| {
        puzzle.split("\n")
        .map(|row| row.chars().collect())
        .collect()
    }).collect()
}

fn rotate_puzzle(puzzle: &Puzzle) -> Puzzle {
    let columns = puzzle[0].len();
    let rows = puzzle.len();
    let mut rotated_puzzle: Puzzle = vec![vec!['X'; rows]; columns];

    for i in 0..rows {
        for j in 0..columns {
            rotated_puzzle[j][rows - 1 - i] = puzzle[i][j]
        }
    }

    rotated_puzzle
}

fn mirror_value(puzzle: &Puzzle) -> u32 {
    let result = value(puzzle);

    if result != 0 {
        return result * 100;
    }
    
    value(&rotate_puzzle(puzzle))
}

fn value(puzzle: &Puzzle) -> u32 {
    for row in 0..puzzle.len() {
        if is_mirror(puzzle, &row) {
            return (row + 1) as u32;
        }
    }
    0
}

fn is_mirror(puzzle: &Puzzle, row_index: &usize) -> bool {
    let mut smudge_found = false;
    let mut row = row_index.clone() as i32;
    let mut mirror_row = row_index + 1;

    while row >= 0 && mirror_row < puzzle.len() {
        for column in 0..puzzle[0].len() {
            if puzzle[row as usize][column] == puzzle[mirror_row][column] {
                continue;
            }

            if smudge_found {
                return false;
            }

            smudge_found = true
        }

        row -= 1;
        mirror_row += 1;
    }
    
    smudge_found
}

 
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1("#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.

#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#");

        assert_eq!(result, "400");
    }

    #[test]
    fn test_horizontal_mirror() {
        let result = mirror_value(&parse_input("#...##..#
#....#..#
..##..###
#####.##.
#####.##.
..##..###
#....#..#")[0]);
        assert_eq!(result, 100)
    }

    #[test]
    fn test_vertical_mirror() {
        let result = mirror_value(&parse_input("#.##..##.
..#.##.#.
##......#
##......#
..#.##.#.
..##..##.
#.#.##.#.")[0]);
        assert_eq!(result, 300)
    }

    #[test]
    fn test_mirror_first_row() {
        let result = mirror_value(&parse_input("#...#.#..##
#...#.#..##
..##.######
..####.....
##.##.###.#
#.###.....#
#.###.#...#")[0]);
        assert_eq!(result, 600)
    }

    #[test]
    fn actual_part1() {
        let input = include_str!("./input.txt");
        let output = part1(input);
        assert_eq!(output, "28475")
    }
}