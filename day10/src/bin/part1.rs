fn main() {
    let input: &str = include_str!("./input.txt");
    let output: String = part1(input);
    dbg!(output);
}

#[derive(Debug)]
struct Pos {
    row: usize,
    column: usize
}

fn part1(input: &str) -> String {
    let map = parse_input(input);

    let mut pos: Pos = find_start(&map);
    let mut direction = find_start_direction(&map, &pos);
    let mut step = 0;

    while step == 0 || map[pos.row][pos.column] != 'S' {
        pos = next_pos(&pos, &direction);
        direction = next_direction(&direction, &map[pos.row][pos.column]);
        step += 1;
    }

    (step / 2).to_string()
}

fn parse_input(input: &str) -> Vec<Vec<char>> {
    let lines: Vec<&str> = input.lines().collect();
    lines.iter().map(|line| line.chars().collect()).collect()
}

fn find_start(map: &Vec<Vec<char>>) -> Pos {
    let mut position = Pos{row: 0, column: 0};
 
    for row_index in 0..map.len() {
        let column_index = map[row_index].iter().position(|p| p == &'S');
        if column_index.is_some() {
            position = Pos{row: row_index, column: column_index.unwrap_or_default()};
            break;
        }
    }

    position
}

fn find_start_direction(map: &Vec<Vec<char>>, start: &Pos) -> char {
    let up = map[start.row - 1][start.column];
    if up == '|' || up == '7' || up == 'F' {
        return 'N'
    }

    let down: char = map[start.row + 1][start.column];
    if down == '|' || down == 'J' || down == 'L' {
        return 'S'
    }

    let west = map[start.row][start.column - 1];
    if west == '-' || west == 'F' || west == 'L' {
        return 'W'
    }

    'E'
}

fn next_pos(pos: &Pos, direction: &char) -> Pos {
    match direction {
        'N' => return Pos{row: pos.row - 1, column: pos.column},
        'S' => return Pos{row: pos.row + 1, column: pos.column},
        'W' => return Pos{row: pos.row, column: pos.column - 1},
        'E' => return Pos{row: pos.row, column: pos.column + 1},
        _default => return Pos{row: pos.row, column: pos.column}
    }
}

fn next_direction(direction: &char, pipe: &char) -> char {
    match pipe {
        '|' => return *direction,
        '-' => return *direction,
        'L' => return if direction == &'S' {'E'} else {'N'},
        'J' => return if direction == &'S' {'W'} else {'N'},
        '7' => return if direction == &'N' {'W'} else {'S'},
        'F' => return if direction == &'N' {'E'} else {'S'},
        _default => *direction
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1("-L|F7
7S-7|
L|7||
-L-J|
L|-JF");

        assert_eq!(result, "4");
    }
    
    #[test]
    fn complex_test_part1() {
        let result = part1("..F7.
.FJ|.
SJ.L7
|F--J
LJ...");

        assert_eq!(result, "8");
    }

    #[test]
    fn actual_part1() {
        let input = include_str!("./input.txt");
        let output = part1(input);
        assert_eq!(output, "6864")
    }
}