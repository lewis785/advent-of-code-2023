use std::collections::HashMap;
use regex::Regex;

fn main() {
    let input: &str = include_str!("./input.txt");
    let output: String = part1(input);
    dbg!(output);
}


type Step = (Box<dyn Fn(&Gear) -> bool>, String);
type Workflows = HashMap<String, Vec<Step>>;

#[derive(Debug)]
struct Gear {
    x: u32,
    m: u32,
    a: u32,
    s: u32
}

impl From<&str> for Gear {
    fn from(value: &str) -> Self {
        let regex = Regex::new(r"(\d+)").unwrap();
        let values: Vec<u32> = regex.captures_iter(value)
        .map(|cap| cap[1].parse::<u32>().unwrap())
        .collect();

        Gear {
            x: values[0],
            m: values[1],
            a: values[2],
            s: values[3]
        }    
    }
}

fn parse_input(input: &str) -> (Workflows, Vec<Gear>)  {
    let (workflow_string, gear_string) = input.split_once("\n\n").unwrap();
    let mut workflows: Workflows = HashMap::new();

    for workflow in workflow_string.lines() {
        let parsed_workflow = workflow.replace("}", "");
        let (key, step_string) = parsed_workflow.split_once('{').unwrap();
        let steps: Vec<Step> = step_string.split(',').map(|step| parse_step(step)).collect();
        workflows.insert(key.to_string(), steps);
    }

    let gears: Vec<Gear> = gear_string.lines().map(|line| Gear::from(line)).collect();

    (workflows, gears)
}

fn parse_step(step: &str) -> Step {
    let split: Vec<&str> = step.split(":").collect();
    if split.len() == 1 {
        return (Box::new(|_gear: &Gear| true), step.to_string());
    }
    
    let regex = Regex::new(r"([xmas])([><])(\d+)").unwrap();
    let output = split[1];
    let check = regex.captures(split[0]).unwrap();
    let value: u32 = check[3].parse::<u32>().unwrap();

    if &check[2] == ">" {
        return match &check[1] {
            "x" => (Box::new(move |gear: &Gear| gear.x > value), output.to_string()),
            "m" => (Box::new(move |gear: &Gear| gear.m > value), output.to_string()),
            "a" => (Box::new(move |gear: &Gear| gear.a > value), output.to_string()),
            "s" => (Box::new(move |gear: &Gear| gear.s > value), output.to_string()),
            _ => panic!("Invalid character")
        };
    }

    match &check[1] {
        "x" => (Box::new(move |gear: &Gear| gear.x < value), output.to_string()),
        "m" => (Box::new(move |gear: &Gear| gear.m < value), output.to_string()),
        "a" => (Box::new(move |gear: &Gear| gear.a < value), output.to_string()),
        "s" => (Box::new(move |gear: &Gear| gear.s < value), output.to_string()),
        _ => panic!("Invalid character")
    }
}

fn part1(input: &str) -> String {
    let (workflows, gears) = parse_input(input);
    let results = gears.iter().map(|gear| gear_status(&workflows, gear)).sum::<u32>();

    results.to_string()
}

fn gear_status(workflows: &Workflows, gear: &Gear) -> u32 {
    let mut workflow_name = "in";

    while workflow_name != "A" && workflow_name != "R" {
        for (check, result) in &workflows[workflow_name] {
            if check(gear) {
                workflow_name = &result;
                break;
            }
        }
    }

    if workflow_name == "A" { gear.x + gear.m + gear.a + gear.s } else { 0 }
} 

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let result = part1("px{a<2006:qkq,m>2090:A,rfg}
pv{a>1716:R,A}
lnx{m>1548:A,A}
rfg{s<537:gd,x>2440:R,A}
qs{s>3448:A,lnx}
qkq{x<1416:A,crn}
crn{x>2662:A,R}
in{s<1351:px,qqz}
qqz{s>2770:qs,m<1801:hdj,R}
gd{a>3333:R,R}
hdj{m>838:A,pv}

{x=787,m=2655,a=1222,s=2876}
{x=1679,m=44,a=2067,s=496}
{x=2036,m=264,a=79,s=2244}
{x=2461,m=1339,a=466,s=291}
{x=2127,m=1623,a=2188,s=1013}");

        assert_eq!(result, "19114");
    }

    #[test]
    fn actual_part1() {
        let input = include_str!("./input.txt");
        let output = part1(input);
        assert_eq!(output, "376008")
    }
}