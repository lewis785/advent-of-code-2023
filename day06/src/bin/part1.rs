fn main() {
    let input: &str = include_str!("./input.txt");
    let output: String = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let lines: Vec<Vec<i32>> = input.lines()
        .map(|line| line.split_once(" ").unwrap().1)
        .map(|line| line.split_whitespace().filter_map(|x| x.parse::<i32>().ok()).collect())
        .collect();

    let races = &lines[0];
    let records = &lines[1];

    let output = races.iter().enumerate().map(|(i, race)| {
        let record = records[i];
        return (0..*race).enumerate()
        .map(|(index, _)| (index as i32) * (race - index as i32))
        .filter(|distance| distance > &record)
        .collect::<Vec<i32>>()
        .len();
        // dbg!(&distances);
    }).fold(1, |output, value| { return value * output});

    return output.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1("Time:      7  15   30
Distance:  9  40  200");

        assert_eq!(result, "288");
    }

    #[test]
    fn actual_part1() {
        let input = include_str!("./input.txt");
        let output = part1(input);
        assert_eq!(output, "512295")
    }
}