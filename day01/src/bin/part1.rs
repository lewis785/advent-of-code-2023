fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let input_array: Vec<Vec<char>> = input.lines().map(|line| line.chars().collect()).collect();

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1("1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet");

        assert_eq!(result, "142");
    }

    #[test]
    fn actual_part1() {
        let input = include_str!("./input.txt");
        let output = part1(input);
        assert_eq!(output, "55816")
    }
}