#[derive(Debug)]
struct Assignment {
    section_start: u32,
    section_end: u32,
}

impl From<&str> for Assignment {
    fn from(value: &str) -> Self {
        let mut parsed = value.split("-").map(|s| s.parse::<u32>().unwrap());
        let left = parsed.next().unwrap();
        let right = parsed.next().unwrap();
        Assignment {
            section_start: left,
            section_end: right,
        }
    }
}

impl Assignment {
    fn contains(&self, other: &Assignment) -> bool {
        self.section_start <= other.section_start && self.section_end >= other.section_end
    }
    fn overlaps(&self, other: &Assignment) -> bool {
        self.contains(&other)
            || other.contains(&self)
            || (self.section_start <= other.section_start
                && self.section_end >= other.section_start)
            || (self.section_start <= other.section_end && self.section_end >= other.section_end)
    }
}

fn main() {
    let test_input = include_str!("../input/day4.test");
    let test_result = part1(test_input);
    println!("test part 1: {}", test_result);

    let prod_input = include_str!("../input/day4.prod");
    let prod_result = part1(prod_input);
    println!("prod part 1: {}", prod_result);

    let test_input = include_str!("../input/day4.test");
    let test_result = part2(test_input);
    println!("test part 2: {}", test_result);

    let prod_input = include_str!("../input/day4.prod");
    let prod_result = part2(prod_input);
    println!("prod part 2: {}", prod_result);
}

fn part2(test_input: &str) -> u32 {
    test_input
        .lines()
        .map(|line| line.split(","))
        .map(|mut pieces| {
            let first_elf = pieces.next().unwrap();
            let second_elf = pieces.next().unwrap();
            (Assignment::from(first_elf), Assignment::from(second_elf))
        })
        .filter(|(first_elf_assignment, second_elf_assignment)| {
            first_elf_assignment.overlaps(&second_elf_assignment)
        })
        .map(|_| 1)
        .sum::<u32>()
}

fn part1(test_input: &str) -> u32 {
    test_input
        .lines()
        .map(|line| line.split(","))
        .map(|mut pieces| {
            let first_elf = pieces.next().unwrap();
            let second_elf = pieces.next().unwrap();
            (Assignment::from(first_elf), Assignment::from(second_elf))
        })
        .filter(|(first_elf_assignment, second_elf_assignment)| {
            return first_elf_assignment.contains(&second_elf_assignment)
                || second_elf_assignment.contains(&first_elf_assignment);
        })
        .map(|_| 1)
        .sum::<u32>()
}
