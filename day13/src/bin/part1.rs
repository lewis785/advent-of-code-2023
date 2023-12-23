fn main() {
    let input: &str = include_str!("./input.txt");
    let output: String = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let puzzles = parse_input(input);

    let output: u32 = puzzles.iter()
    .map(|puzzle| mirror_value(puzzle))
    .sum();
    
    output.to_string()
}

fn parse_input(input: &str) -> Vec<Vec<String>> {
    let puzzles: Vec<&str> = input.split("\n\n").collect();

    puzzles.iter()
    .map(|puzzle| puzzle.split("\n").map(String::from).collect())
    .collect()
}

fn rotate_puzzle(puzzle: &Vec<String>) -> Vec<String> {

    let matrix: Vec<Vec<char>> = puzzle.iter().map(|row| row.chars().collect()).collect();
    let columns = matrix[0].len();
    let rows = matrix.len();
    let mut rotated_puzzle: Vec<Vec<char>> = vec![vec!['X'; rows]; columns];

    for i in 0..rows {
        for j in 0..columns {
            rotated_puzzle[j][rows - 1 - i] = matrix[i][j]
        }
    }

    rotated_puzzle.iter().map(|row| row.iter().collect::<String>()).collect()
}

fn mirror_value(puzzle: &Vec<String>) -> u32 {
    let result = value(puzzle);

    if result != 0 {
        return result * 100;
    }
    
    value(&rotate_puzzle(puzzle))
}

fn value(puzzle: &Vec<String>) -> u32 {
    let mut stack: Vec<&str> = Vec::new();

    for (index, row) in puzzle.iter().enumerate() {
        if index + 1 >= puzzle.len() {
            break
        }
        
        stack.push(row);

        if row == &puzzle[index + 1] && is_perfect_mirror(&stack, &puzzle[index+1..]) {
            return stack.len() as u32
        }
    }

    0
}

fn is_perfect_mirror(existing: &Vec<&str>, remaining: &[String]) -> bool {
    for (index , current) in existing.iter().rev().enumerate() {
        if index >= remaining.len() {
            break;
        }
        if current != &remaining[index] {
            return false
        }
    }

    true
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

        assert_eq!(result, "405");
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
        assert_eq!(result, 400)
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
        assert_eq!(result, 5)
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
        assert_eq!(result, 100)
    }

    #[test]
    fn test_is_perfect_mirror() {
        let result = is_perfect_mirror(&vec!["A", "B", "C"], &["C", "B", "A"].map(String::from));
        
        assert_eq!(result, true)
    }

    #[test]
    fn test_is_perfect_mirror_longer() {
        let result = is_perfect_mirror(&vec!["D", "A", "B", "C"], &["C", "B", "A"].map(String::from));
        
        assert_eq!(result, true)
    }

    #[test]
    fn test_is_not_perfect_mirror() {
        let result = is_perfect_mirror(&vec!["A", "B"], &["C", "B", "A"].map(String::from));
        
        assert_eq!(result, false)
    }

    #[test]
    fn actual_part1() {
        let input = include_str!("./input.txt");
        let output = part1(input);
        assert_eq!(output, "33356")
    }
}