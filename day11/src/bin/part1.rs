fn main() {
    let input: &str = include_str!("./input.txt");
    let output: String = part1(input);
    dbg!(output);
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos (i32, i32);

type Map = Vec<Vec<char>>;

fn part1(input: &str) -> String {
    let map = parse_input(input);
    let expanded_map = expand_map(&map);
    let galaxy_pos = find_galaxies(&expanded_map);
    let galaxy_combination = galaxy_combinations(&galaxy_pos);

    galaxy_combination.iter()
    .map(|(start, end)| {
        shortest_path( start, end)
    })
    .sum::<u32>()
    .to_string()
}

fn parse_input(input: &str) -> Map {
    input.lines()
    .map(|line| line.chars().collect())
    .collect()
}

fn expand_map(map: &Map) -> Vec<Vec<char>> {
    let mut expanded_width: Vec<Vec<char>> = vec![Vec::new(); map.len()];

    for i in 0..map[0].len() {
        if map.iter().all(|row| row[i] == '.') {
            expanded_width = expanded_width.iter().map(|row| { 
                let mut new_row = row.clone();
                new_row.push('.');
                new_row.push('.');
                new_row
            }).collect();
            continue;
        }

        expanded_width = expanded_width.iter().enumerate().map(|(row_index, row)| {
            let mut new_row = row.clone();
            new_row.push(map[row_index][i]);
            new_row
        }).collect();
    }


    expanded_width.iter().fold(Vec::new(), |mut output, row| {
        if row.iter().all(|cell| cell == &'.') {
            output.push(row.clone());
        }
        output.push(row.clone());
        output
    })
}

fn find_galaxies(map: &Map) -> Vec<Pos> {
    let mut galaxies = Vec::<Pos>::new();

    map.iter().enumerate().for_each(|(row_index, row)| {
        row.iter().enumerate().for_each(|(column_index, cell)| {
            if cell == &'#' {
                galaxies.push(Pos(column_index as i32, row_index as i32))
            }
        })
    });

    galaxies
}

fn galaxy_combinations(galaxies: &Vec<Pos>) -> Vec<(Pos, Pos)> {
    let mut combinations = Vec::new();

    for i in 0..galaxies.len() {
        for j in i+1..galaxies.len() {
            combinations.push((galaxies[i].clone(), galaxies[j].clone()));
        }
    }
    
    combinations
}

fn shortest_path(start: &Pos, end: &Pos) -> u32 {
    start.0.abs_diff(end.0) + start.1.abs_diff(end.1)
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1("...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....");

        assert_eq!(result, "374");
    }

    #[test]
    fn actual_part1() {
        let input = include_str!("./input.txt");
        let output = part1(input);
        assert_eq!(output, "10490062")
    }
}