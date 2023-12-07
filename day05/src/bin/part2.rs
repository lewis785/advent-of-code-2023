fn main() {
    let input: &str = include_str!("./input.txt");
    let output: String = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    let (seeds, maps) = input.split_once("\n\n").unwrap();
    let seed_numbers: Vec<(i64, i64)> = convert_seed_string(seeds);
    let maps_output: Vec<Vec<Vec<i64>>> = convert_map_string(maps);

    dbg!(seed_numbers);

    // let output = maps_output.iter().fold(seed_numbers, |values, map| {
    //     return map_to_next(&values, map);
    // });

    // return output.iter().min().unwrap().to_string();   
    return "Test".to_string();
}

fn convert_seed_string(seeds_string: &str) -> Vec<(i64, i64)> {
   let seed_list: Vec<i64> = seeds_string.replace("seeds: ", "")
    .split_whitespace()
    .filter_map(|x| x.parse::<i64>().ok())
    .collect();

    return seed_list.iter().enumerate().fold(Vec::new(), |mut output, (index, _)| {
        if index + index + 1 >= seed_list.len() {
            return output
        }
        
        output.push((seed_list[index + index], seed_list[index + index + 1]));
        return output
    });
}


fn convert_map_string(map_string: &str) -> Vec<Vec<Vec<i64>>> {
    let maps: Vec<&str> = map_string.split("\n\n").collect();
    let mut maps_list: Vec<Vec<Vec<i64>>> = Vec::new();

    maps.iter().for_each(|map| {
        let rows: Vec<&str> = map.split_once(":\n").unwrap().1.lines().collect();
        let parsed_rows: Vec<Vec<i64>> = rows.iter()
                            .map(|row| row.split_whitespace()
                            .filter_map(|x| x.parse::<i64>().ok()).collect())
                            .collect();
        maps_list.push(parsed_rows);
    });

    return maps_list
}

fn map_to_next(inputs: &Vec<(i64, i64)>, map: &Vec<Vec<i64>>) -> Vec<i64> {
    return inputs.iter().map(|input| {
        let value = map.iter().fold(-1, |output: i64, row: &Vec<i64>| {
            
        });

        if value == -1 { return *input } else { return value }
    }).collect();
}

fn maps_to((input, range): (i64, i64), map: Vec<i64>) -> Vec<(i64, i64)> {
    let destination = map[0];
    let start = map[1];
    let range = map[2];
    let upper_input = input + range - 1;

    if upper_input < start {
        return vec![(input, range)]
    }

    if input < start && upper_input > start {
        let mut result = vec![(input, start - input)];
        result.append(maps_to((start, upper_input - start), map));
        return result;
    }

    return vec![]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let result = part2("seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4");

        assert_eq!(result, "46");
    }

    // #[test]
    // fn actual_part2() {
    //     let input = include_str!("./input.txt");
    //     let output = part2(input);
    //     assert_eq!(output, "340994526")
    // }
}