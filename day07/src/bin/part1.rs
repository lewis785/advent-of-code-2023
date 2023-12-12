use std::collections::HashMap;

fn main() {
    let input: &str = include_str!("./input.txt");
    let output: String = part1(input);
    dbg!(output);
}

#[derive(Debug)]
struct Play {
    hand: String,
    bet: i32,
    play: u8
}

fn part1(input: &str) -> String {
    let lines: Vec<&str> = input.lines().collect();
    let mut plays: Vec<Play> = lines.iter().map(|line| {
        let (hand, bet) = line.split_once(" ").unwrap();
        Play{hand: String::from(hand), bet: bet.parse::<i32>().unwrap_or_default(), play: get_play_type(hand)}
    }).collect();


    plays.sort_by(|a, b| {
        if &b.play == &a.play {
            return highest_card(&a.hand, &b.hand);
        }
        a.play.cmp(&b.play)
    });

    dbg!(&plays);
    
    let output: i32 = plays.iter().enumerate()
    .map(|(index, play)| { play.bet * (index + 1 ) as i32})
    .sum();

    output.to_string()
}

fn get_play_type(hand: &str) -> u8 {
    let hand_hash = convert_to_hash(hand);

    // Five of a kind
    if hand_hash.len() == 1 {
        return 6
    }

    let counts: Vec<&i8> = hand_hash.values().collect();

    if counts.len() == 2 {
        // Four of a kind
        if counts[0] == &4 || counts[1] == &4 {
            return 5
        }
        // Full house
        return 4
    }

    // Three of a kind
    if counts.iter().filter(|x| x == &&&3).collect::<Vec<_>>().len() == 1 {
        return 3
    }

    let pair_count = counts.iter().filter(|x| x == &&&2).collect::<Vec<_>>().len();

    if pair_count == 2 {
        return 2
    }

    if pair_count == 1 {
        return 1
    }

    return 0
}

fn convert_to_hash(hand: &str) -> HashMap<char, i8> {
    let mut hash = HashMap::new();

    hand.chars().for_each(|card| {
        if hash.contains_key(&card) {
            hash.insert(card, hash[&card] + 1);
            return 
        }
        hash.insert(card, 1);
    });

    hash
}

fn highest_card(a: &str, b: &str) -> std::cmp::Ordering {
    let a_chars: Vec<char> = a.chars().collect();
    let b_chars: Vec<char> = b.chars().collect();
    
    for x in  0..=a.len() {
        if &a_chars[x] == &b_chars[x] {
            continue;
        }
        return card_to_number(&a_chars[x]).cmp(&card_to_number(&b_chars[x]))
    }

    0.cmp(&0)
}

fn card_to_number(card: &char) -> i32 {
    let mut map = HashMap::new();
    map.insert('A', 14);
    map.insert('K', 13);
    map.insert('Q', 12);
    map.insert('J', 11);
    map.insert('T', 10);
    map.insert('9', 9);
    map.insert('8', 8);
    map.insert('7', 7);
    map.insert('6', 6);
    map.insert('5', 5);
    map.insert('4', 4);
    map.insert('3', 3);
    map.insert('2', 2);
    
    map[card]
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