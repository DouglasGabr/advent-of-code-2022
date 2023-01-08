fn main() {
    let test_input = include_str!("../input/day8/test.txt");
    let prod_input = include_str!("../input/day8/prod.txt");

    let test_part1_result = part1(test_input);
    println!("part 1 test: {}", test_part1_result);

    let prod_part1_result = part1(prod_input);
    println!("part 1 prod: {}", prod_part1_result);

    let test_part2_result = part2(test_input);
    println!("part 2 test: {}", test_part2_result);

    let prod_part2_result = part2(prod_input);
    println!("part 2 prod: {}", prod_part2_result);
}

fn parse_input(input: &str) -> Vec<Vec<u32>> {
    input
        .lines()
        .map(|line| {
            line.chars()
                .map(|c| c.to_digit(10).expect("only valid digit chars"))
                .collect()
        })
        .collect()
}

#[derive(Copy, Clone)]
enum Direction {
    Up,
    Down,
    Left,
    Right,
}

fn count_visible_trees(lines: &Vec<Vec<u32>>, source: (usize, usize), direction: Direction) -> u32 {
    let range: Box<dyn Iterator<Item = usize>> = match direction {
        Direction::Up => Box::new((0..source.0).rev()),
        Direction::Down => Box::new(source.0 + 1..lines.len()),
        Direction::Left => Box::new((0..source.1).rev()),
        Direction::Right => Box::new(source.1 + 1..lines[0].len()),
    };
    let mut valid_tree_count = 0;
    let tree_height = lines[source.0][source.1];
    for i in range {
        let other_tree_height = match direction {
            Direction::Up | Direction::Down => lines[i][source.1],
            Direction::Left | Direction::Right => lines[source.0][i],
        };
        valid_tree_count += 1;
        if other_tree_height >= tree_height {
            break;
        }
    }
    valid_tree_count
}

fn part2(input: &str) -> u32 {
    let lines = parse_input(input);

    lines
        .iter()
        .enumerate()
        .map(|(x, line)| {
            line.iter()
                .enumerate()
                .map(|(y, _)| {
                    [
                        Direction::Up,
                        Direction::Right,
                        Direction::Down,
                        Direction::Left,
                    ]
                    .iter()
                    .map(|direction| count_visible_trees(&lines, (x, y), *direction))
                    .reduce(|a, b| a * b)
                    .expect("iterator should have at least 1 entry")
                })
                .max()
                .expect("iterator is not empty")
        })
        .max()
        .expect("iterator is not empty")
}

fn part1(input: &str) -> u32 {
    let lines = parse_input(input);

    let mut visible_trees =
        (lines.len() * lines[0].len()) - ((lines.len() - 2) * (lines[0].len() - 2));

    for x in 1..lines.len() - 1 {
        let line = &lines[x];
        for y in 1..line.len() - 1 {
            let tree_height = line[y];
            if (0..x).all(|dx| {
                let other_tree_height = lines[dx][y];
                other_tree_height < tree_height
            }) || (x + 1..lines.len()).all(|dx| {
                let other_tree_height = lines[dx][y];
                other_tree_height < tree_height
            }) || (0..y).all(|dy| {
                let other_tree_height = lines[x][dy];
                other_tree_height < tree_height
            }) || (y + 1..line.len()).all(|dy| {
                let other_tree_height = lines[x][dy];
                other_tree_height < tree_height
            }) {
                visible_trees += 1;
            }
        }
    }

    visible_trees.try_into().unwrap()
}
