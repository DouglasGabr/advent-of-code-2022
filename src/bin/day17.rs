use std::{
    collections::BTreeSet,
    fmt::{Display, Write},
    iter::Cycle,
};

type Position = (usize, usize);

#[derive(Clone, Copy)]
struct Rock {
    body: &'static [Position],
    is_falling: bool,
}

impl Rock {
    fn width(&self) -> usize {
        self.body.iter().map(|(_, col)| col).max().unwrap() + 1
    }
    fn height(&self) -> usize {
        self.body.iter().map(|(row, _)| row).max().unwrap() + 1
    }
}

impl Display for Rock {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let width = self.width();
        let height = self.height();
        let mut grid = vec![vec!['.'; width]; height];
        for (row, col) in self.body {
            grid[*row][*col] = if self.is_falling { '@' } else { '#' };
        }
        for row in grid.iter().rev() {
            for col in row {
                f.write_str(&col.to_string())?;
            }
            f.write_str("\n")?;
        }
        Ok(())
    }
}

const ROCKS: [Rock; 5] = [
    Rock {
        body: &[(0, 0), (0, 1), (0, 2), (0, 3)],
        is_falling: false,
    },
    Rock {
        body: &[(0, 1), (1, 0), (1, 1), (1, 2), (2, 1)],
        is_falling: false,
    },
    Rock {
        body: &[(2, 2), (1, 2), (0, 0), (0, 1), (0, 2)],
        is_falling: false,
    },
    Rock {
        body: &[(0, 0), (1, 0), (2, 0), (3, 0)],
        is_falling: false,
    },
    Rock {
        body: &[(0, 0), (0, 1), (1, 0), (1, 1)],
        is_falling: false,
    },
];

#[derive(Clone, Copy)]
enum Jet {
    Left,
    Right,
}

impl From<char> for Jet {
    fn from(s: char) -> Self {
        match s {
            '<' => Jet::Left,
            '>' => Jet::Right,
            _ => panic!("Invalid jet"),
        }
    }
}

impl Display for Jet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Jet::Left => "<",
            Jet::Right => ">",
        })
    }
}

struct Chamber<JetIter: Clone + Iterator<Item = Jet>, RockIter: Clone + Iterator<Item = Rock>> {
    width: usize,
    rocks: Vec<(Position, Rock)>,
    jet_pattern: Cycle<JetIter>,
    rocks_to_drop: Cycle<RockIter>,
    spawned_rocks: usize,
}

fn get_rock_occupied_positions_in_chamber((offset, rock): &(Position, Rock)) -> BTreeSet<Position> {
    return rock
        .body
        .iter()
        .map(|(row, col)| (row + offset.0, col + offset.1))
        .collect();
}

impl<JetIter: Clone + Iterator<Item = Jet>, RockIter: Clone + Iterator<Item = Rock>>
    Chamber<JetIter, RockIter>
{
    fn new(width: usize, jet_pattern: Cycle<JetIter>, rocks_to_drop: Cycle<RockIter>) -> Self {
        let mut chamber = Chamber {
            width,
            rocks: vec![],
            jet_pattern,
            rocks_to_drop,
            spawned_rocks: 0,
        };
        chamber.spawn_rock();
        chamber
    }

    fn spawn_rock(&mut self) {
        let rock = self.rocks_to_drop.next().unwrap();
        let offset = (self.highest_point() + 3, 2);
        self.rocks.push((
            offset,
            Rock {
                is_falling: true,
                ..rock
            },
        ));
        self.spawned_rocks += 1;
    }

    fn highest_point(&self) -> usize {
        self.rocks
            .iter()
            .map(|((row, _), rock)| row + rock.height())
            .max()
            .unwrap_or(0)
    }

    fn highest_point_of_settled_rocks(&self) -> usize {
        self.rocks
            .iter()
            .filter(|(_, rock)| !rock.is_falling)
            .map(|((row, _), rock)| row + rock.height())
            .max()
            .unwrap_or(0)
    }

    fn tick(&mut self) {
        let current_occupied_positions = self.get_current_occupied_positions();
        let mut falling_rock = self.rocks.last_mut().unwrap();
        assert!(falling_rock.1.is_falling, "last rock isn't falling");
        let jet = self.jet_pattern.next().unwrap();
        let falling_rock_occupied_positions = get_rock_occupied_positions_in_chamber(falling_rock);
        match jet {
            Jet::Left => {
                if falling_rock.0 .1 > 0 {
                    if falling_rock_occupied_positions
                        .iter()
                        .map(|(row, col)| (*row, col - 1))
                        .all(|new_position| !current_occupied_positions.contains(&new_position))
                    {
                        falling_rock.0 .1 -= 1;
                    }
                }
            }
            Jet::Right => {
                if falling_rock.0 .1 + falling_rock.1.width() < self.width {
                    if falling_rock_occupied_positions
                        .iter()
                        .map(|(row, col)| (*row, col + 1))
                        .all(|new_position| !current_occupied_positions.contains(&new_position))
                    {
                        falling_rock.0 .1 += 1;
                    }
                }
            }
        }
        if falling_rock.0 .0 == 0 {
            falling_rock.1.is_falling = false;
            self.trim();
            self.spawn_rock();
            return;
        }
        let falling_rock_occupied_positions = get_rock_occupied_positions_in_chamber(falling_rock);
        if falling_rock_occupied_positions
            .iter()
            .map(|(row, col)| (row - 1, *col))
            .any(|new_position| current_occupied_positions.contains(&new_position))
        {
            falling_rock.1.is_falling = false;
            self.trim();
            self.spawn_rock();
            return;
        }
        falling_rock.0 .0 -= 1;
    }

    fn get_current_occupied_positions(&mut self) -> BTreeSet<(usize, usize)> {
        let current_occupied_positions = self
            .rocks
            .iter()
            .filter(|(_, rock)| !rock.is_falling)
            .flat_map(get_rock_occupied_positions_in_chamber)
            .collect::<BTreeSet<Position>>();
        current_occupied_positions
    }

    fn trim(&mut self) {
        let occupied_positions = self.get_current_occupied_positions();
        let highest_point = self.highest_point_of_settled_rocks();
        for row in (0..highest_point).rev() {
            if (0..self.width)
                .map(|col| (row, col))
                .all(|position| occupied_positions.contains(&position))
            {
                self.rocks = self
                    .rocks
                    .clone()
                    .into_iter()
                    .filter(|(offset, rock)| rock.is_falling || offset.0 + rock.height() >= row)
                    .collect()
            }
        }
    }
}

impl<JetIter: Clone + Iterator<Item = Jet>, RockIter: Clone + Iterator<Item = Rock>> Display
    for Chamber<JetIter, RockIter>
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut grid = vec![vec!['.'; self.width]; self.highest_point()];
        for ((row, col), rock) in &self.rocks {
            for (r, c) in rock.body {
                grid[row + r][col + c] = if rock.is_falling { '@' } else { '#' };
            }
        }
        for row in grid.iter().rev() {
            f.write_char('|')?;
            for col in row {
                f.write_str(&col.to_string())?;
            }
            f.write_char('|')?;
            f.write_str("\n")?;
        }
        f.write_str(("+".to_string() + &"-".repeat(self.width) + "+\n").as_str())?;
        Ok(())
    }
}

fn main() {
    let input = include_str!("../input/day17.txt");
    println!("part 1");
    part1(input);
    println!("part 2");
    part2(input);
}

fn part1(jet_pattern: &str) {
    let mut chamber = Chamber::new(
        7,
        jet_pattern.chars().map(Jet::from).cycle(),
        ROCKS.into_iter().cycle(),
    );
    while chamber.spawned_rocks <= 2022 {
        chamber.tick();
    }
    println!("{}", chamber.highest_point_of_settled_rocks());
}
fn part2(jet_pattern: &str) {
    let mut chamber = Chamber::new(
        7,
        jet_pattern.chars().map(Jet::from).cycle(),
        ROCKS.into_iter().cycle(),
    );
    while chamber.spawned_rocks <= 1000000000000 {
        chamber.tick();
        if chamber.spawned_rocks % 100_000 == 0 {
            println!("{} / {}", chamber.rocks.len(), chamber.spawned_rocks);
        }
    }
    println!("{}", chamber.highest_point_of_settled_rocks());
}

#[cfg(test)]
mod test {

    use super::*;

    #[test]
    fn test() {
        let jet_pattern = ">>><<><>><<<>><>>><<<>>><<<><<<>><>><<>>";
        part1(jet_pattern);
    }

    #[test]
    fn rocks_are_correct() {
        for rock in ROCKS {
            println!("{}", rock);
        }
    }

    #[test]
    fn spawn_rock() {
        let chamber = Chamber::new(
            7,
            vec![Jet::Left, Jet::Right].into_iter().cycle(),
            ROCKS.into_iter().cycle(),
        );
        println!("{}", chamber);
    }

    #[test]
    fn tick() {
        let mut chamber = Chamber::new(
            7,
            vec![Jet::Left, Jet::Right].into_iter().cycle(),
            ROCKS.into_iter().cycle(),
        );
        chamber.tick();
        println!("{}", chamber);
    }
}
