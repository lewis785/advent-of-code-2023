use std::collections::HashMap;

fn main() {
    let input: &str = include_str!("./input.txt");
    let output: String = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    
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
        let result = part1("-L|F7
7S-7|
L|7||
-L-J|
L|-JF");

        assert_eq!(result, "8");
    }

    #[test]
    fn actual_part1() {
        let input = include_str!("./input.txt");
        let output = part1(input);
        assert_eq!(output, "512295")
    }
}