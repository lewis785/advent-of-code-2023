use std::cmp::{max, min};

fn main() {
    let input: &str = include_str!("./input.txt");
    let output: String = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let lines: std::str::Lines<'_> = input.lines();
    let grid: Vec<Vec<char>> = lines.map(|line: &str| line.chars().collect()).collect();
    let width = grid[0].len();

    let mut parts: Vec<i32> = Vec::new();

    for (i, row) in grid.iter().enumerate() {
        let mut number_string = String::new();
        let mut is_adjacent = false;

        for (j, _) in row.iter().enumerate() {
            let character = grid[i][j];
            if character.is_numeric() {
                number_string.push(character);
            }

            if character.is_numeric() && adjacent_to_symbol(&grid, i, j) {
                is_adjacent = true
            }

            if character.is_numeric() && j+1 < width {
                continue
            }

            if is_adjacent {
                dbg!(&number_string);
                parts.push(number_string.parse::<i32>().unwrap());
            }

            number_string = String::new();
            is_adjacent = false;
        }
    }

    return parts.iter().sum::<i32>().to_string();
}

fn adjacent_to_symbol(grid: &Vec<Vec<char>>, row: usize, column: usize) -> bool {
    let height = (grid.len() - 1)  as isize;
    let width = (grid[0].len() - 1) as isize;

    return [
        is_not_number_or_period(grid[max((row as isize) - 1, 0) as usize][max((column as isize) - 1, 0) as usize]),
        is_not_number_or_period(grid[max((row as isize) - 1, 0) as usize][column]),
        is_not_number_or_period(grid[max((row as isize) - 1, 0) as usize][min((column as isize) + 1, width) as usize]),
        is_not_number_or_period(grid[row][max((column as isize) - 1, 0) as usize]),
        is_not_number_or_period(grid[row][min((column as isize) + 1, width) as usize]),
        is_not_number_or_period(grid[min((row as isize) + 1, height) as usize][max((column as isize) - 1, 0) as usize]),
        is_not_number_or_period(grid[min((row as isize) + 1, height) as usize][column]),
        is_not_number_or_period(grid[min((row as isize) + 1, height) as usize][min((column as isize) + 1, width) as usize]),
    ].contains(&true);
}

fn is_not_number_or_period(char: char) -> bool {
    return !char.is_alphanumeric() && char != '.'
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1("467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..");

        assert_eq!(result, "4361");
    }

    #[test]
    fn actual_part1() {
        let input = include_str!("./input.txt");
        let output = part1(input);
        assert_eq!(output, "540212")
    }
}