fn main() {
    let input: &str = include_str!("./input.txt");
    let output: String = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let sequences = parse_input(input);

    sequences.iter()
    .map(|sequence| generate_next_value(sequence))
    .sum::<i64>()
    .to_string()
}

fn parse_input(input: &str) -> Vec<Vec<i64>> {
    let lines: Vec<&str> = input.lines().collect();
    lines.iter().map(|line| {
        line.split_ascii_whitespace()
        .filter_map(|number| number.parse::<i64>().ok())
        .collect()
    }).collect()
}

fn generate_next_value(sequence: &Vec<i64>) -> i64 {
    if sequence.iter().all(|value| value == &0 ) {
        return 0
    }

    let next_sequence = next_sequence(sequence);

    sequence.last().unwrap_or(&0) + generate_next_value(&next_sequence)
}

fn next_sequence(sequence: &Vec<i64>) -> Vec<i64> {
    sequence.iter().enumerate().fold(Vec::new(), |mut output, (index, value)| {
        if index + 1 > sequence.len() - 1 {
            return output
        }
        output.push(sequence[index+1] - value);

        output
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1("0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45");

        assert_eq!(result, "114");
    }

    #[test]
    fn test_predict_when_sequence_starts_negative() {
        let result = generate_next_value(&vec![-6, -3, 0, 3, 6, 9]);
        assert_eq!(result, 12)
    }

    #[test]
    fn test_negative_sequence() {
        let result = generate_next_value(&vec![1, -1, -3, -5, -7, -9, -11, -13, -15, -17, -19, -21, -23, -25, -27, -29, -31, -33, -35, -37, -39]);
        assert_eq!(result, -41)
    }
    
    #[test]
    fn actual_part1() {
        let input = include_str!("./input.txt");
        let output = part1(input);
        assert_eq!(output, "1743490457")
    }
}