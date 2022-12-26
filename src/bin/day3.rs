use std::collections::HashSet;

fn main() {
    let input = include_str!("../input/day3.txt");
    let sum = input
        .lines()
        .map(|contents| {
            let length = contents.len();
            let mut first_rucksack = HashSet::<u32>::new();
            let mut second_rucksack = HashSet::<u32>::new();
            for (index, char) in contents.chars().enumerate() {
                let parsed_weight = parse_char_weight(char);
                if index < (length / 2) {
                    first_rucksack.insert(parsed_weight);
                } else {
                    second_rucksack.insert(parsed_weight);
                }
            }
            let intersection = first_rucksack.intersection(&second_rucksack);
            let content_sum = intersection.sum::<u32>();
            content_sum
        })
        .sum::<u32>();
    println!("part 1: {:?}", sum);

    let sum = input
        .lines()
        .collect::<Vec<&str>>()
        .chunks(3)
        .map(|chunk| {
            let mut first_badges = HashSet::<u32>::new();
            let mut second_badges = HashSet::<u32>::new();
            let mut third_badges = HashSet::<u32>::new();
            for (index, contents) in chunk.iter().enumerate() {
                for char in contents.chars() {
                    let parsed_weight = parse_char_weight(char);
                    match index {
                        0 => first_badges.insert(parsed_weight),
                        1 => second_badges.insert(parsed_weight),
                        2 => third_badges.insert(parsed_weight),
                        _ => panic!("invalid index"),
                    };
                }
            }
            let intersection = first_badges
                .intersection(&second_badges)
                .map(|weight| *weight);
            HashSet::<u32>::from_iter(intersection)
                .intersection(&third_badges)
                .map(|weight| *weight)
                .next()
                .unwrap_or(0)
        })
        .sum::<u32>();
    println!("part 2: {:?}", sum);
}

fn parse_char_weight(char: char) -> u32 {
    let parsed_weight = char.to_digit(36).unwrap() - 9 + (if char.is_uppercase() { 26 } else { 0 });
    parsed_weight
}
