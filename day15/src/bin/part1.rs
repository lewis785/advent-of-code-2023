fn main() {
    let input: &str = include_str!("./input.txt");
    let output: String = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let words: Vec<&str> = input.split(',').collect();
    
    words.iter().map(|word| hash(word)).sum::<u32>().to_string()
}


fn hash(word: &str) -> u32  {
    let word_bytes = word.as_bytes();
    let mut hash: u32 = 0;

    for byte in word_bytes {
        hash += *byte as u32;
        hash *= 17;
        hash %= 256;
    }

    hash
}

// fn hash()

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7");

        assert_eq!(result, "1320");
    }

    #[test]
    fn test_hash() {
        let result = hash("HASH");

        assert_eq!(result, 52)
    }

    #[test]
    fn actual_part1() {
        let input = include_str!("./input.txt");
        let output = part1(input);
        assert_eq!(output, "512295")
    }
}