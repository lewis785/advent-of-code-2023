fn main() {
    let input = include_str!("./input.txt");
    let output = part1(input);
    dbg!(output);
}

struct Game {
    blue: u32,
    green: u32,
    red: u32,
}

fn part1(input: &str) -> String {
    let lines = input.lines();

    
    let output = lines.enumerate().map(|(i,  line)| {
        let result = line.to_string().split( ':').nth(1).unwrap_or_default().to_string();
        let colours = result.replace(";", ",");
 
        let game = colours.split(',').fold(Game {blue: 0, green: 0, red: 0}, |game, result| {
            let mut parts = result.split_whitespace();
            let value = parts.next().unwrap_or_default().parse::<u32>().unwrap();
            let colour = parts.next().unwrap_or_default().to_string();

            
            if colour == "red" && value > game.red {
                return Game { red: value, green: game.green, blue: game.blue} 
            }
            if colour == "green" && value > game.green {
                return Game { red: game.red, green: value, blue: game.blue} 
            }
            if colour == "blue" && value > game.blue {
                return Game { red: game.red, green: game.green, blue: value} 
            }
            return game
        });

        if game.red <= 12 && game.green <= 13 && game.blue <= 14 { (i as u32) + 1 } else { 0 }
    }).sum::<u32>();

    return output.to_string();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1("Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green");

        assert_eq!(result, "8");
    }

    #[test]
    fn actual_part1() {
        let input = include_str!("./input.txt");
        let output = part1(input);
        assert_eq!(output, "2512")
    }
}