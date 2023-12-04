fn main() {
    let input: &str = include_str!("./input.txt");
    let output: String = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    let lines: std::str::Lines<'_> = input.lines();
    let grid: Vec<Vec<char>> = lines.map(|line: &str| line.chars().collect()).collect();

    let mut parts: Vec<i32> = Vec::new();

    for (i, row) in grid.iter().enumerate() {
        for (j, _) in row.iter().enumerate() {
            let character = grid[i][j];
            if character != '*' {
                continue
            }

            parts.push(get_gear_ratio(&grid, i, j))
        }
    }

    return parts.iter().sum::<i32>().to_string();
}

fn get_gear_ratio(grid: &Vec<Vec<char>>, row: usize, column: usize) -> i32 {
    let height = (grid.len() - 1)  as isize;
    let width = (grid[0].len() - 1) as isize;
    let mut unique_numbers: Vec<i32> = Vec::new();
    let ranges = [
        -1, 0 , 1
    ];

    
    for row_diff in ranges {
        let mut last_number_column = -10;

        for column_diff in ranges {
            let new_row = (row as isize + row_diff).min(height).max(0) as usize;
            let new_column = (column as isize + column_diff).min(width).max(0) as usize;

            if grid[new_row][new_column].is_numeric() &&  (new_column == 0 || (new_column as isize) - 1 != last_number_column ) {
                unique_numbers.push(get_number(&grid[new_row], new_column));
                last_number_column = new_column as isize;
            }

            if grid[new_row][new_column].is_numeric() &&  (new_column as isize) - 1 == last_number_column {
                last_number_column = new_column as isize;
            }
        }
    }

    if unique_numbers.len() != 2 {
        return 0
    }

    return unique_numbers.iter().fold(1, |output, value| value * output)
}

fn get_number(row: &Vec<char>, column: usize) -> i32 {
    let width = row.len() - 1;
    let mut index = column as isize;
    let mut number_string = String::new();
    
    while index - 1 >= 0 && row[(index - 1) as usize].is_numeric() {
        index -= 1;
    }

    while index as usize <= width {
        if !row[index as usize].is_numeric() {
            break;
        }
        number_string.push(row[index as usize]) ;
        index += 1;
    }

    return number_string.parse::<i32>().unwrap();
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let result = part2("467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..");

        assert_eq!(result, "467835");
    }

    #[test]
    fn same_number_part2() {
        let result = part2(".35..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..");

        assert_eq!(result, "452715");
    }

    #[test]
    fn actual_part2() {
        let input = include_str!("./input.txt");
        let output = part2(input);
        assert_eq!(output, "87605697")
    }
}