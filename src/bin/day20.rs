fn main() {
    let input = include_str!("../input/day20.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn mix(original: &[i64], data: &mut Vec<usize>) {
    for (original_index, &number) in original.iter().enumerate() {
        let data_index = data
            .iter()
            .enumerate()
            .find_map(|(index, original_ref_index)| {
                (*original_ref_index == original_index).then_some(index)
            })
            .expect("Index exists in data");
        let new_index = (data_index as i64 + number) % (data.len() - 1) as i64;
        let new_index = match new_index {
            ..=0 if number.is_negative() => data.len() as i64 + new_index - 1,
            _ => new_index,
        } as usize;
        data.remove(data_index);
        data.insert(new_index, original_index);
    }
}

fn calculate_result(original: &[i64], data: &[usize]) -> i64 {
    let original_zero_index = original
        .iter()
        .enumerate()
        .find_map(|(index, &number)| (number == 0).then_some(index))
        .expect("Index exists in data");
    let base_index = data
        .iter()
        .enumerate()
        .find_map(|(index, &original_index)| {
            (original_index == original_zero_index).then_some(index)
        })
        .expect("index exists in data");
    [1000, 2000, 3000]
        .into_iter()
        .map(|weight| (weight + base_index) % data.len())
        .map(|index| data[index])
        .map(|original_index| original[original_index])
        .sum()
}

fn parse_input(input: &str, decryption_key: i64) -> Vec<i64> {
    input
        .lines()
        .map(|line| line.parse::<i64>().unwrap() * decryption_key)
        .collect()
}

fn part1(input: &str) -> i64 {
    let original = parse_input(input, 1);
    let mut data = (0..original.len()).collect::<Vec<_>>();
    mix(&original, &mut data);
    calculate_result(&original, &data)
}

const PART_2_DECRYPTION_KEY: i64 = 811589153;
fn part2(input: &str) -> i64 {
    let original = parse_input(input, PART_2_DECRYPTION_KEY);
    let mut data = (0..original.len()).collect::<Vec<_>>();
    for _ in 0..10 {
        mix(&original, &mut data);
    }
    calculate_result(&original, &data)
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST_DATA: &'static str = "1\n2\n-3\n3\n-2\n0\n4";

    #[test]
    fn test_part_1() {
        assert_eq!(part1(TEST_DATA), 3);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part2(TEST_DATA), 1623178306);
    }
}
