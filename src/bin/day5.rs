#[derive(Clone, Default)]
struct Stack {
    crates: Vec<char>,
}
struct Ship {
    stacks: Vec<Stack>,
}

impl Ship {
    fn move_crates(&mut self, count: u32, from: usize, to: usize) {
        let source_index = from - 1;
        let destination_index = to - 1;
        let source_stack = self.stacks.get_mut(source_index).unwrap();
        let mut crates_to_move = source_stack
            .crates
            .split_off(source_stack.crates.len() - count as usize);
        crates_to_move.reverse();
        let destination_stack = self.stacks.get_mut(destination_index).unwrap();
        destination_stack.crates.append(&mut crates_to_move);
    }

    fn move_crates_at_once(&mut self, count: u32, from: usize, to: usize) {
        let source_index = from - 1;
        let destination_index = to - 1;
        let source_stack = self.stacks.get_mut(source_index).unwrap();
        let mut crates_to_move = source_stack
            .crates
            .split_off(source_stack.crates.len() - count as usize);
        let destination_stack = self.stacks.get_mut(destination_index).unwrap();
        destination_stack.crates.append(&mut crates_to_move);
    }
}

fn main() {
    let test_input = include_str!("../input/day5.test");
    let test_result = part1(test_input);
    println!("test part 1: {}", test_result);

    let prod_input = include_str!("../input/day5.prod");
    let prod_result = part1(prod_input);
    println!("prod part 1: {}", prod_result);

    let test_result = part2(test_input);
    println!("test part 2: {}", test_result);

    let prod_result = part2(prod_input);
    println!("prod part 2: {}", prod_result);
}

fn part1(input: &str) -> String {
    let Some((initial_state, instructions)) = input.split_once("\n\n") else {
        panic!("Invalid input");
    };
    let number_of_stacks = (initial_state.lines().next().unwrap().len() + 1) / 4;
    let mut ship = Ship {
        stacks: vec![Stack::default(); number_of_stacks],
    };
    for line in initial_state.lines() {
        let mut chars = line.chars();
        chars.next(); // skip box border
        for (stack_index, crate_char) in chars.step_by(4).enumerate() {
            if !crate_char.is_alphabetic() {
                continue;
            }
            ship.stacks
                .get_mut(stack_index)
                .expect("Invalid stack index")
                .crates
                .push(crate_char);
        }
    }
    for stack in ship.stacks.iter_mut() {
        stack.crates.reverse();
    }
    for instruction in instructions.lines() {
        let mut parts = instruction.split_whitespace();
        parts.next().unwrap(); // skip "move"
        let count = parts.next().unwrap().parse::<u32>().unwrap();
        parts.next().unwrap(); // skip "from"
        let from = parts.next().unwrap().parse::<usize>().unwrap();
        parts.next().unwrap(); // skip "to"
        let to = parts.next().unwrap().parse::<usize>().unwrap();
        ship.move_crates(count, from, to);
    }
    return ship
        .stacks
        .iter()
        .flat_map(|stack| stack.crates.last())
        .collect();
}

fn part2(input: &str) -> String {
    let Some((initial_state, instructions)) = input.split_once("\n\n") else {
        panic!("Invalid input");
    };
    let number_of_stacks = (initial_state.lines().next().unwrap().len() + 1) / 4;
    let mut ship = Ship {
        stacks: vec![Stack::default(); number_of_stacks],
    };
    for line in initial_state.lines() {
        let mut chars = line.chars();
        chars.next(); // skip box border
        for (stack_index, crate_char) in chars.step_by(4).enumerate() {
            if !crate_char.is_alphabetic() {
                continue;
            }
            ship.stacks
                .get_mut(stack_index)
                .expect("Invalid stack index")
                .crates
                .push(crate_char);
        }
    }
    for stack in ship.stacks.iter_mut() {
        stack.crates.reverse();
    }
    for instruction in instructions.lines() {
        let mut parts = instruction.split_whitespace();
        parts.next().unwrap(); // skip "move"
        let count = parts.next().unwrap().parse::<u32>().unwrap();
        parts.next().unwrap(); // skip "from"
        let from = parts.next().unwrap().parse::<usize>().unwrap();
        parts.next().unwrap(); // skip "to"
        let to = parts.next().unwrap().parse::<usize>().unwrap();
        ship.move_crates_at_once(count, from, to);
    }
    return ship
        .stacks
        .iter()
        .flat_map(|stack| stack.crates.last())
        .collect();
}
