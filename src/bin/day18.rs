use std::collections::BTreeSet;

fn main() {
    let input = include_str!("../input/day18.txt");
    println!("Part 1: {}", part1(input));
}

fn part1(input: &str) -> usize {
    let droplets = input
        .lines()
        .map(|line| {
            let mut pieces = line.split(",");
            let x = pieces.next().expect("x").parse::<i64>().expect("number");
            let y = pieces.next().expect("y").parse::<i64>().expect("number");
            let z = pieces.next().expect("z").parse::<i64>().expect("number");
            (x, y, z)
        })
        .collect::<BTreeSet<_>>();
    return droplets
        .iter()
        .map(|&(x, y, z)| {
            [
                (x - 1, y, z),
                (x + 1, y, z),
                (x, y - 1, z),
                (x, y + 1, z),
                (x, y, z - 1),
                (x, y, z + 1),
            ]
            .into_iter()
            .filter(|neighbor| droplets.contains(neighbor))
            .count()
        })
        .map(|neighbors| 6 - neighbors)
        .sum();
}

type Coordinate = (i64, i64, i64);

fn part2(input: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;
    const TEST_INPUT: &str = "2,2,2\n\
                              1,2,2\n\
                              3,2,2\n\
                              2,1,2\n\
                              2,3,2\n\
                              2,2,1\n\
                              2,2,3\n\
                              2,2,4\n\
                              2,2,6\n\
                              1,2,5\n\
                              3,2,5\n\
                              2,1,5\n\
                              2,3,5";
    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 64);
    }
    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 58);
    }
}
