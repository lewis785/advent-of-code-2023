fn main() {
    let input: &str = include_str!("./input.txt");
    let output: String = part2(input);
    dbg!(output);
}

fn part2(input: &str) -> String {
    let lines = input.lines();
    let scratch_cards: Vec<Vec<&str>> = lines.map(|line| line.split_once(": ").unwrap().1.split(" | ").collect()).collect();
    
    let mut card_counts = vec![1; scratch_cards.len()];


    scratch_cards.iter().enumerate().for_each( |(i, scratch_cards)| {
        let winning_number: Vec<&str> = scratch_cards[0].split_whitespace().collect();
        let actual_numbers: Vec<&str> = scratch_cards[1].split_whitespace().collect();
        let card_count = card_counts[i];

        let wins = actual_numbers.iter().filter(|x| winning_number.contains(x)).collect::<Vec<_>>().len();
        
        for x in 0..wins {
            card_counts[i + x + 1] += card_count
        }
    });

    return card_counts.iter().sum::<i32>().to_string()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part2() {
        let result = part2("Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11");

        assert_eq!(result, "30");
    }

    #[test]
    fn actual_part2() {
        let input = include_str!("./input.txt");
        let output = part2(input);
        assert_eq!(output, "6420979")
    }
}