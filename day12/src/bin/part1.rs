fn main() {
    let input: &str = include_str!("./input.txt");
    let output: String = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let arrangements = parse_input(input);
    let output: Vec<u32> = arrangements.iter().map(|(arrangement, rules)| {
        combinations(arrangement, rules)
    }).collect();

    output.iter().sum::<u32>().to_string()
}

fn parse_input(input: &str) -> Vec<(Vec<char>, Vec<u32>)> {
    let lines: Vec<&str> = input.lines().collect();

    lines.iter().map(|line| {
        let (map, rules) = line.split_once(' ').unwrap();
        let parsed_rules: Vec<u32> = rules.split(',').filter_map(|r| r.parse::<u32>().ok()).collect();

        (map.chars().collect(), parsed_rules)
    }).collect()
}

fn combinations(input: &Vec<char>, rules: &Vec<u32>) -> u32 {
    if !valid_map(input, rules) {
        return 0
    }

    if !input.iter().any(|cell| cell == &'?') {
        return 1
    }

    if let Some(unknown_index) = input.iter().position(|i| i == &'?') {
        let mut left = input.clone();
        let mut right = input.clone();
        left[unknown_index] = '.';
        right[unknown_index] = '#';

        combinations(&left, rules) + combinations(&right, rules)
    } else {
        0
    }
}

fn valid_map(map: &Vec<char>, rules: &Vec<u32>) -> bool {
    let mut rule:usize = 0;
    let mut skip = 0;
    let mut passed_rules = false;

    for i in 0..map.len() {
        if map[i] == '?' {
            return true
        }

        if skip > 0 { 
            skip -= 1;
            if skip > 0 {
                continue
            }

            if map[i] == '#' {
                return false
            }

            continue;
        }


        if map[i] == '.'  {
            continue;
        }

        if passed_rules {
            return false
        }

        let end_of_sequence = i + (rules[rule] as usize);

        if end_of_sequence > map.len() || !valid_sequence(&map[i..end_of_sequence]) {
            return false
        }

        skip = rules[rule];
        rule += 1;
        passed_rules = rule >= rules.len()
    }

    rule >= rules.len()
}

fn valid_sequence(sequence: &[char]) -> bool {
    !sequence.iter().any(|x| x == &'.')
}
 
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1("???.### 1,1,3
.??..??...?##. 1,1,3
?#?#?#?#?#?#?#? 1,3,1,6
????.#...#... 4,1,1
????.######..#####. 1,6,5
?###???????? 3,2,1");

        assert_eq!(result, "21");
    }

    #[test]
    fn test_combinations_one() {
        let result = combinations(&vec!['?','?','?','.','#','#','#'], &vec![1,1,3]);
        assert_eq!(result, 1)
    }

    #[test]
    fn test_combinations_two() {
        let result = combinations(&vec!['.','?','?','.','.','?','?','.','.','.','?','#','#','.'], &vec![1,1,3]);
        assert_eq!(result, 4)
    }

    #[test]
    fn test_combinations_three() {
        let result = combinations(&vec!['?','#','#','#','?','?','?','?','?','?','?','?'], &vec![3,2,1]);
        assert_eq!(result, 10)
    }

    #[test]
    fn actual_part1() {
        let input = include_str!("./input.txt");
        let output = part1(input);
        assert_eq!(output, "7344")
    }
}