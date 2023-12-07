fn main() {
    let input: &str = include_str!("./input.txt");
    let output: String = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    let lines: Vec<Vec<i64>> = input.lines()
        .map(|line| line.split_once(" ").unwrap().1.replace(" ", ""))
        .map(|line| line.split_whitespace().filter_map(|x| x.parse::<i64>().ok()).collect())
        .collect();

    dbg!(&lines);
    
    let races = &lines[0];
    let records = &lines[1];

    let output = races.iter().enumerate().map(|(i, race)| {
        let record = records[i];
        return (0..*race).enumerate()
        .map(|(index, _)| (index as i64) * (race - index as i64))
        .filter(|distance| distance > &record)
        .collect::<Vec<i64>>()
        .len();
        // dbg!(&distances);
    }).fold(1, |output, value| { return value * output});

    return output.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let result = part2("Time:      7  15   30
Distance:  9  40  200");

        assert_eq!(result, "71503");
    }

    #[test]
    fn actual_part2() {
        let input = include_str!("./input.txt");
        let output = part2(input);
        assert_eq!(output, "36530883")
    }
}