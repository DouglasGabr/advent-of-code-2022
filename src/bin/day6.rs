fn main() {
    let test_part1_input = include_str!("../input/day6.test");
    let test_part1_result = part1(test_part1_input);
    println!("test part 1: {}", test_part1_result);

    let prod_part1_input = include_str!("../input/day6.prod");
    let prod_part1_result = part1(prod_part1_input);
    println!("prod part 1: {}", prod_part1_result);

    let test_part2_input = include_str!("../input/day6.test");
    let test_part2_result = part2(test_part2_input);
    println!("test part 2: {}", test_part2_result);

    let prod_part2_input = include_str!("../input/day6.prod");
    let prod_part2_result = part2(prod_part2_input);
    println!("prod part 2: {}", prod_part2_result);
}

fn part1(input: &str) -> usize {
    process_input::<4>(input)
}

fn part2(input: &str) -> usize {
    process_input::<14>(input)
}

fn process_input<const NUM_OF_DISTINCT_CHARS: usize>(input: &str) -> usize {
    let mut last_n_chars = ['.'; NUM_OF_DISTINCT_CHARS];
    'input_loop: for (index, char) in input.chars().enumerate() {
        let sub_index = index % NUM_OF_DISTINCT_CHARS;
        last_n_chars[sub_index] = char;
        for (char_index, char_to_compare) in last_n_chars.iter().enumerate() {
            if !char_to_compare.is_alphabetic() {
                continue 'input_loop;
            }
            for other_char in last_n_chars.iter().skip(char_index + 1) {
                if char_to_compare == other_char {
                    continue 'input_loop;
                }
            }
        }
        return index + 1;
    }
    0
}
