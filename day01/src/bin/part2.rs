fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let input_array: Vec<Vec<char>> = replace_words_with_numbers(input).lines().map(|line| line.chars().collect()).collect();

    let output: i32 = input_array.iter().map(|line: &Vec<char>| {
        let first = line.iter().find(|c| c.is_numeric());
        let last = line.iter().rev().find(|c| c.is_numeric());
        let start_str = first.map_or_else(|| String::new(), |c| c.to_string());
        let end_str = last.map_or_else(|| String::new(), |c| c.to_string());
        let merged_str = start_str + &end_str;
        merged_str.parse::<i32>().unwrap_or(0)
    }).sum();

    return output.to_string();
}

fn replace_words_with_numbers(input: &str) -> String {
    return input.replace("one", "o1e")
        .replace("two", "t2o")
        .replace("three", "t3e")
        .replace("four", "f4r")
        .replace("five", "f5e")
        .replace("six", "s6x")
        .replace("seven", "s7n")
        .replace("eight", "e8t")
        .replace("nine", "n9e");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let result = part1("two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen");

        assert_eq!(result, "281");
    }

    #[test]
    fn actual_part1() {
        let input = include_str!("./input.txt");
        let output = part1(input);
        assert_eq!(output, "54980")
    }
}