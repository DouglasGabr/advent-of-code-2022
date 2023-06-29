use std::{
    array::IntoIter,
    collections::{
        hash_map::Entry::{Occupied, Vacant},
        HashMap, HashSet,
    },
    fmt::Display,
    iter::Cycle,
};

type Point = (i64, i64);

#[derive(Clone)]
enum Direction {
    North,
    South,
    West,
    East,
}

impl Direction {
    fn move_point(&self, point: &Point) -> Point {
        match self {
            Direction::North => (point.0 + 1, point.1),
            Direction::South => (point.0 - 1, point.1),
            Direction::West => (point.0, point.1 - 1),
            Direction::East => (point.0, point.1 + 1),
        }
    }
    fn get_proposed_positions(&self, ref_pos: &(i64, i64)) -> [(i64, i64); 3] {
        match self {
            Direction::North => [
                (ref_pos.0 + 1, ref_pos.1 - 1),
                (ref_pos.0 + 1, ref_pos.1),
                (ref_pos.0 + 1, ref_pos.1 + 1),
            ],
            Direction::South => [
                (ref_pos.0 - 1, ref_pos.1 - 1),
                (ref_pos.0 - 1, ref_pos.1),
                (ref_pos.0 - 1, ref_pos.1 + 1),
            ],
            Direction::West => [
                (ref_pos.0 - 1, ref_pos.1 - 1),
                (ref_pos.0, ref_pos.1 - 1),
                (ref_pos.0 + 1, ref_pos.1 - 1),
            ],
            Direction::East => [
                (ref_pos.0 - 1, ref_pos.1 + 1),
                (ref_pos.0, ref_pos.1 + 1),
                (ref_pos.0 + 1, ref_pos.1 + 1),
            ],
        }
    }
}

struct Grid {
    elves: HashSet<Point>,
    proposed_direction_cycle: Cycle<IntoIter<Direction, 4>>,
}

impl Grid {
    fn new(elves: HashSet<Point>) -> Self {
        Grid {
            elves,
            proposed_direction_cycle: [
                Direction::North,
                Direction::South,
                Direction::West,
                Direction::East,
            ]
            .into_iter()
            .cycle(),
        }
    }

    fn edges(&self) -> (Point, Point) {
        let left_bottom_point = self
            .elves
            .iter()
            .fold((i64::MAX, i64::MAX), |cur, elf_pos| {
                (cur.0.min(elf_pos.0), cur.1.min(elf_pos.1))
            });
        let right_top_point = self
            .elves
            .iter()
            .fold((i64::MIN, i64::MIN), |cur, elf_pos| {
                (cur.0.max(elf_pos.0), cur.1.max(elf_pos.1))
            });
        (left_bottom_point, right_top_point)
    }

    fn round(&mut self) -> usize {
        let elves = self.elves.clone();
        let elves_with_neighbors_iter = elves.iter().filter(|&elf_position| {
            let neighbors = (elf_position.0 - 1..=elf_position.0 + 1)
                .flat_map(|row| {
                    (elf_position.1 - 1..=elf_position.1 + 1).map(move |col| (row, col))
                })
                .filter(|pos| *pos != *elf_position)
                .filter(|pos| self.elves.contains(pos))
                .count();
            neighbors > 0
        });
        let proposed_moves = elves_with_neighbors_iter
            .flat_map(|elf| -> Option<(&Point, Point)> {
                for direction in self.proposed_direction_cycle.clone().take(4) {
                    let proposed_positions = direction.get_proposed_positions(elf);
                    if proposed_positions
                        .iter()
                        .all(|pos| !self.elves.contains(pos))
                    {
                        return Some((elf, direction.move_point(elf)));
                    }
                }
                None
            })
            .fold(
                HashMap::<Point, Vec<&Point>>::new(),
                |mut acc, (elf_pos, proposed_pos)| {
                    acc.entry(proposed_pos)
                        .and_modify(|elves| elves.push(elf_pos))
                        .or_insert(vec![elf_pos]);
                    acc
                },
            );
        let moves_iter = proposed_moves
            .into_iter()
            .filter(|(_, elves)| elves.len() == 1)
            .map(|(destination, elves)| (destination, elves[0]));
        let mut moved = 0;
        for (destination, elf_pos) in moves_iter {
            self.elves.remove(elf_pos);
            self.elves.insert(destination);
            moved += 1;
        }
        self.proposed_direction_cycle.next();
        moved
    }

    fn count_empty_tiles_covered(&self) -> usize {
        let (left_bottom, top_right) = self.edges();
        (left_bottom.0..=top_right.0)
            .flat_map(|row| (left_bottom.1..=top_right.1).map(move |col| (row, col)))
            .filter(|pos| !self.elves.contains(pos))
            .count()
    }
}

impl Display for Grid {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (left_bottom, top_right) = self.edges();
        for row in (left_bottom.0..=top_right.0).rev() {
            for col in left_bottom.1..=top_right.1 {
                if self.elves.contains(&(row, col)) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl From<&str> for Grid {
    fn from(input: &str) -> Self {
        let mut elves = HashSet::new();
        for (row, line) in input.lines().rev().enumerate() {
            for (col, tile) in line.chars().enumerate() {
                if tile == '#' {
                    elves.insert((row as i64, col as i64));
                }
            }
        }
        Grid::new(elves)
    }
}

fn main() {
    let input = include_str!("../input/day23.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> usize {
    let mut grid = Grid::from(input);
    for _ in 0..10 {
        grid.round();
    }
    grid.count_empty_tiles_covered()
}

fn part2(input: &str) -> usize {
    let mut grid = Grid::from(input);
    let mut round = 1;
    while grid.round() > 0 {
        round += 1;
    }
    round
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &'static str = "....#..\n\
                                      ..###.#\n\
                                      #...#.#\n\
                                      .#...##\n\
                                      #.###..\n\
                                      ##.#.##\n\
                                      .#..#..";

    #[test]
    fn parse_grid() {
        let grid = Grid::from(TEST_INPUT);
        println!("{}", grid);
    }

    #[test]
    fn grid_edges() {
        let grid = Grid::from(TEST_INPUT);
        let (left_bottom, right_top) = grid.edges();
        assert_eq!(left_bottom, (0, 0));
        assert_eq!(right_top, (6, 6));
    }

    #[test]
    fn grid_round() {
        let mut grid = Grid::from(TEST_INPUT);
        println!("{}", grid);
        for _ in 0..10 {
            println!("----------------\n");
            grid.round();
            println!("{}", grid);
        }
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(TEST_INPUT), 110);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(TEST_INPUT), 20);
    }
}
