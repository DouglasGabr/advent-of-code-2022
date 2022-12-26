use std::collections::HashMap;

fn main() {
    let input = include_str!("../input/day2.txt");
    let mut strategy_map = HashMap::new();
    strategy_map.insert("A X", 1 + 3);
    strategy_map.insert("A Y", 2 + 6);
    strategy_map.insert("A Z", 3 + 0);

    strategy_map.insert("B X", 1 + 0);
    strategy_map.insert("B Y", 2 + 3);
    strategy_map.insert("B Z", 3 + 6);

    strategy_map.insert("C X", 1 + 6);
    strategy_map.insert("C Y", 2 + 0);
    strategy_map.insert("C Z", 3 + 3);

    let total = input
        .split("\n")
        .flat_map(|round| strategy_map.get(round))
        .sum::<u32>();

    println!("part 1: {:?}", total);

    let mut correct_strategy_map = HashMap::new();
    correct_strategy_map.insert("A X", 3 + 0);
    correct_strategy_map.insert("A Y", 1 + 3);
    correct_strategy_map.insert("A Z", 2 + 6);

    correct_strategy_map.insert("B X", 1 + 0);
    correct_strategy_map.insert("B Y", 2 + 3);
    correct_strategy_map.insert("B Z", 3 + 6);

    correct_strategy_map.insert("C X", 2 + 0);
    correct_strategy_map.insert("C Y", 3 + 3);
    correct_strategy_map.insert("C Z", 1 + 6);

    let total = input
        .split("\n")
        .flat_map(|round| correct_strategy_map.get(round))
        .sum::<u32>();

    println!("part 2: {:?}", total);
}
