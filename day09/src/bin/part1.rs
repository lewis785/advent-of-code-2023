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
        let result = part1("32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483");

        assert_eq!(result, "6440");
    }

    #[test]
    fn actual_part1() {
        let input = include_str!("./input.txt");
        let output = part1(input);
        assert_eq!(output, "512295")
    }
}