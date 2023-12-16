fn main() {
    let input: &str = include_str!("./input.txt");
    let output: String = part2(input, &1000000);
    dbg!(output);
}

#[derive(Clone, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
struct Pos (i32, i32);

type Map = Vec<Vec<char>>;

fn part2(input: &str, scale: &u64) -> String {
    let map = parse_input(input);
    let expansion_points = find_expansion_spots(&map);
    // let expanded_map = expand_map(&map, scale);
    let galaxy_pos = find_galaxies(&map);
    let galaxy_combination = galaxy_combinations(&galaxy_pos);

    galaxy_combination.iter()
    .map(|(start, end)| {
        shortest_path(scale, &expansion_points, start, end)
    })
    .sum::<u64>()
    .to_string()
}

fn parse_input(input: &str) -> Map {
    input.lines()
    .map(|line| line.chars().collect())
    .collect()
}

fn find_expansion_spots(map: &Map) -> (Vec::<u64>, Vec::<u64>) {
    let mut columns = Vec::<u64>::new();

    for i in 0..map[0].len() {
        if map.iter().all(|row| row[i] == '.') {
            columns.push(i as u64);
            continue;
        }
    }

    let rows: Vec<u64> =  map.iter().enumerate().fold(Vec::new(), |mut output, (index, row)| {
        if row.iter().all(|cell| cell == &'.') {
            output.push(index as u64)
        }
        output
    });

    return (columns, rows)
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

fn shortest_path(scale: &u64, expansion_points: &(Vec::<u64>, Vec::<u64>), start: &Pos, end: &Pos) -> u64 {
    let distance = (start.0.abs_diff(end.0) + start.1.abs_diff(end.1)) as u64;

    let width_expanse: Vec<&u64> = expansion_points.0.iter().filter(|p| {
        let a = if start.0 > end.0 { end } else { start };
        let b = if start.0 > end.0 { start } else { end };

        p > &&(a.0 as u64) && p < &&(b.0 as u64)
    }).collect();

    let column_expanse: Vec<&u64> = expansion_points.1.iter().filter(|p| {
        p > &&(start.1 as u64) && p < &&(end.1 as u64)
    }).collect();

    distance + (column_expanse.len() as u64 * (scale - 1)) + (width_expanse.len() as u64 * (scale - 1))
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let result = part2("...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....", &2);

        assert_eq!(result, "374");
    }

    #[test]
    fn test_part2_ten_scale() {
        let result = part2("...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....", &10);

        assert_eq!(result, "1030");
    }

    #[test]
    fn test_part2_hundred_scale() {
        let result = part2("...#......
.......#..
#.........
..........
......#...
.#........
.........#
..........
.......#..
#...#.....", &100);

        assert_eq!(result, "8410");
    }

    #[test]
    fn actual_part2() {
        let input = include_str!("./input.txt");
        let output = part2(input, &1000000);
        assert_eq!(output, "382979724122")
    }
}