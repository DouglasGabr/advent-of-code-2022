use std::collections::{HashMap, HashSet};

fn main() {
    let test_input = include_str!("../input/day12/test.txt");
    let prod_input = include_str!("../input/day12/prod.txt");

    let part_1_test_result = part1(test_input);
    println!("Part 1 test result: {}", part_1_test_result);
    let part_1_prod_result = part1(prod_input);
    println!("Part 1 prod result: {}", part_1_prod_result);

    let part_2_test_result = part2(test_input);
    println!("Part 2 test result: {}", part_2_test_result);
    let part_2_prod_result = part2(prod_input);
    println!("Part 2 prod result: {}", part_2_prod_result);
}

fn part1(input: &str) -> u32 {
    let grid = parse_input(input);
    let (start, end) = grid.find_start_and_end();
    grid.find_shortest_path(start, end).expect("no path found")
}

fn part2(input: &str) -> u32 {
    let grid = parse_input(input);
    let (_, end) = grid.find_start_and_end();
    let starts = grid.find_all_positions_with_char('a');
    starts
        .iter()
        .flat_map(|start| grid.find_shortest_path(*start, end))
        .min()
        .unwrap()
}

enum CellType {
    Start,
    End,
}

struct Cell(u8, Option<CellType>);

struct Grid(Vec<Vec<Cell>>);

impl Grid {
    fn find_all_positions_with_char(&self, c: char) -> Vec<(usize, usize)> {
        self.0
            .iter()
            .enumerate()
            .flat_map(|(x, row)| {
                row.iter()
                    .enumerate()
                    .filter_map(move |(y, cell)| {
                        if cell.0 == c as u8 - b'a' {
                            Some((x, y))
                        } else {
                            None
                        }
                    })
                    .collect::<Vec<_>>()
            })
            .collect()
    }
    fn find_start_and_end(&self) -> ((usize, usize), (usize, usize)) {
        let mut start: Option<(usize, usize)> = None;
        let mut end: Option<(usize, usize)> = None;
        for (x, row) in self.0.iter().enumerate() {
            for (y, cell) in row.iter().enumerate() {
                match cell.1 {
                    Some(CellType::Start) => start = Some((x, y)),
                    Some(CellType::End) => end = Some((x, y)),
                    _ => (),
                }
            }
        }
        (start.unwrap(), end.unwrap())
    }

    fn find_shortest_path(&self, start: (usize, usize), end: (usize, usize)) -> Option<u32> {
        let mut distances = HashMap::new();
        distances.insert(start, 0);

        let mut unvisited = HashSet::new();
        self.0.iter().enumerate().for_each(|(x, row)| {
            row.iter().enumerate().for_each(|(y, _)| {
                unvisited.insert((x, y));
            })
        });
        unvisited.remove(&start);

        let mut shortest_path = vec![Some(start)];

        while !shortest_path.contains(&Some(end)) && !shortest_path.iter().all(Option::is_none) {
            for index in 0..shortest_path.len() {
                let Some(current_position) = shortest_path[index] else {
                    continue;
                };
                let adjacent = [
                    (Some(current_position.0), current_position.1.checked_sub(1)),
                    (Some(current_position.0), current_position.1.checked_add(1)),
                    (current_position.0.checked_sub(1), Some(current_position.1)),
                    (current_position.0.checked_add(1), Some(current_position.1)),
                ];
                let current_cell = &self.0[current_position.0][current_position.1];
                let shortest_adjacent = adjacent
                    .iter()
                    .filter_map(|(x, y)| x.zip(*y))
                    .filter_map(|(x, y)| {
                        let Some(cell) = self.0.get(x).and_then(|row| row.get(y)) else {
                            return None;
                        };
                        if !unvisited.contains(&(x, y)) {
                            return None;
                        }
                        if cell.0 > current_cell.0 + 1 {
                            return None;
                        }
                        let current_distance = distances
                            .get(&current_position)
                            .expect("current cell position should be known");
                        let new_cell_distance = current_distance + 1;
                        Some((
                            (x, y),
                            *distances
                                .entry((x, y))
                                .and_modify(|e| {
                                    if *e > new_cell_distance {
                                        *e = new_cell_distance;
                                    }
                                })
                                .or_insert(new_cell_distance),
                        ))
                    })
                    .min_by(|(_, a), (_, b)| a.cmp(b))
                    .map(|item| item.0);
                match shortest_adjacent {
                    Some(shortest_adjacent) => {
                        unvisited.remove(&shortest_adjacent);
                        shortest_path.push(Some(shortest_adjacent));
                    }
                    None => {
                        shortest_path[index] = None;
                    }
                }
            }
        }

        distances.get(&end).copied()
    }
}

fn parse_input(input: &str) -> Grid {
    Grid(
        input
            .lines()
            .map(|line| {
                line.chars()
                    .map(|c| match c {
                        'S' => Cell(b'a' - b'a', Some(CellType::Start)),
                        'E' => Cell(b'z' - b'a', Some(CellType::End)),
                        other => Cell(other as u8 - b'a', None),
                    })
                    .collect()
            })
            .collect(),
    )
}
