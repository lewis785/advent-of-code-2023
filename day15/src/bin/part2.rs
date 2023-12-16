use std::{collections::HashMap, f32::consts::PI};

fn main() {
    let input: &str = include_str!("./input.txt");
    let output: String = part2(input);
    dbg!(output);
}

type BoxMap = HashMap<usize, Box>;
type Box = Vec<Lens>;
type Lens = (String, u8);

fn part2(input: &str) -> String {
    let mut boxes: BoxMap = HashMap::new();

    input.split(',').for_each(|instruction| {
        let (label, command, focus) = split_instruction(instruction);
        if command == '=' {
            boxes = insert_replace_lens(&boxes, &(label, focus))
        } else {
            boxes = remove_lens(&boxes, &label)
        }
    });
    
    boxes.iter().map(|(key, lenses)| {
        calculate_focus_power(&(*key as u32), lenses)
    }).sum::<u32>().to_string()

    // words.iter().map(|word| hash(word)).sum::<u32>().to_string()
}

fn insert_replace_lens(boxes: &BoxMap, lens: &Lens) -> BoxMap {
    let mut new_boxes = boxes.clone();
    let (label, _) = lens;
    let hash = hash(&label);
    let selected_box = new_boxes.entry(hash as usize).or_insert_with(Vec::new);

    let existing_lens = selected_box.iter().position(|(current_label, _)| current_label == label);
    if let Some(index) = existing_lens {
        selected_box[index] = lens.clone();
    } else {
        selected_box.push(lens.clone());
    }
    
    new_boxes
}

fn remove_lens(boxes: &BoxMap, label: &str) -> BoxMap {
    let mut updated_boxes = boxes.clone();
    let hash = hash(label);

    let selected_box = updated_boxes.entry(hash as usize).or_insert_with(Vec::new);

    if selected_box.len() == 0 {
        return boxes.clone();
    }

    let existing_lens = selected_box.iter().position(|(current_label, _)| current_label == label);
    if let Some(index) = existing_lens {
        selected_box.remove(index);
    }

    updated_boxes
}

fn calculate_focus_power(box_number: &u32, lenses: &Vec<Lens>) -> u32 {
    lenses.iter().enumerate().map(|(index, (_, focus))| {
        (box_number + 1) * ((index as u32) + 1) * (*focus as u32)
    }).sum::<u32>()
}

fn split_instruction(instruction: &str) -> (String, char, u8) {
    let split: Vec<_> = instruction.split('=').collect();
    if split.len() == 2 {
        return (split[0].to_string(), '=', split[1].parse::<u8>().unwrap())
    } 

    (split[0].to_string().replace('-', ""), '-', 0)
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
    fn test_part2() {
        let result = part2("rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7");

        assert_eq!(result, "145");
    }

    #[test]
    fn actual_part2() {
        let input = include_str!("./input.txt");
        let output = part2(input);
        assert_eq!(output, "290779")
    }
}