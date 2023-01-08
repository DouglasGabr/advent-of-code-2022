use std::{
    collections::{HashMap, HashSet},
    fmt::Display,
};

fn main() {
    let input = include_str!("../input/day14/prod.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> u32 {
    let mut map = Map::from(input);
    loop {
        if !map.drop_sand_unit(false) {
            break;
        }
    }
    map.get_number_of_sand_at_rest()
}

fn part2(input: &str) -> u32 {
    let mut map = Map::from(input);
    for _ in 0.. {
        if !map.drop_sand_unit(true) {
            break;
        }
    }
    map.get_number_of_sand_at_rest()
}

enum Tile {
    Rock,
    Sand,
}

impl Display for Tile {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Tile::Rock => '#',
                Tile::Sand => 'o',
            }
        )
    }
}

struct Map {
    tiles: HashMap<(u32, u32), Tile>,
    floor_level: u32,
}

impl Map {
    fn get_number_of_sand_at_rest(&self) -> u32 {
        self.tiles
            .iter()
            .filter(|tile| matches!(tile.1, Tile::Sand))
            .count() as u32
    }

    fn drop_sand_unit(&mut self, has_floor: bool) -> bool {
        let first_y_hit = self.tiles.iter().map(|tile| tile.0 .1).min().unwrap();
        let mut sand_position = (500, first_y_hit - 1);
        'falling: while sand_position.1 <= self.floor_level {
            let new_y = sand_position.1 + 1;
            if !(has_floor && new_y == self.floor_level) {
                let possible_positions = [
                    (sand_position.0, new_y),
                    (sand_position.0 - 1, new_y),
                    (sand_position.0 + 1, new_y),
                ];
                for possible_position in possible_positions {
                    if !self.tiles.contains_key(&possible_position) {
                        sand_position = possible_position;
                        continue 'falling;
                    }
                }
            }
            self.tiles.insert(sand_position, Tile::Sand);
            return sand_position != (500, 0);
        }
        false
    }
}

impl Display for Map {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let min_x = self.tiles.iter().map(|tile| tile.0 .0).min().unwrap();
        let min_y = 0;
        let max_x = self.tiles.iter().map(|tile| tile.0 .0).max().unwrap();
        let max_y = self.tiles.iter().map(|tile| tile.0 .1).max().unwrap();
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                if let Some(tile) = self.tiles.get(&(x, y)) {
                    write!(f, "{}", tile)?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }
        Ok(())
    }
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        let tiles: HashMap<(u32, u32), Tile> = value
            .lines()
            .flat_map(|line| {
                let path: Vec<(u32, u32)> = line
                    .split(" -> ")
                    .filter_map(|coords| coords.split_once(','))
                    .map(|(x, y)| {
                        (
                            x.parse::<u32>().expect("valid x"),
                            y.parse::<u32>().expect("valid y"),
                        )
                    })
                    .collect();
                path.windows(2)
                    .flat_map(|path_slice| {
                        let (x1, y1) = path_slice[0];
                        let (x2, y2) = path_slice[1];
                        let (x1, x2) = (x1.min(x2), x1.max(x2));
                        let (y1, y2) = (y1.min(y2), y1.max(y2));
                        (x1..=x2).flat_map(move |x| (y1..=y2).map(move |y| (x, y)))
                    })
                    .collect::<HashSet<(u32, u32)>>()
            })
            .map(|coord| (coord, Tile::Rock))
            .collect();
        Map {
            floor_level: tiles.keys().map(|tile| tile.1).max().unwrap() + 2,
            tiles,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../input/day14/test.txt");
        assert_eq!(part1(input), 24);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../input/day14/test.txt");
        assert_eq!(part2(input), 93);
    }
}
