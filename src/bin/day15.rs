use std::{collections::HashMap, fmt::Display, mem::Discriminant};

fn main() {}

fn part1() -> u32 {
    todo!()
}

fn part2() -> u32 {
    todo!()
}

#[derive(PartialEq, Debug)]
enum Tile {
    Sensor,
    Beacon,
}

#[derive(PartialEq, Debug)]
struct Map {
    tiles: HashMap<(i32, i32), Tile>,
}

// x=0, y=11
fn parse_location(loc_str: &str) -> (i32, i32) {
    loc_str
        .split_once(", ")
        .and_then(|(x, y)| Some((x.split_once('=')?, y.split_once('=')?)))
        .and_then(|(x, y)| Some((x.1.parse().ok()?, y.1.parse().ok()?)))
        .unwrap()
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        Map {
            tiles: value
                .lines()
                .filter_map(|line| {
                    line.split_once(": closest beacon is at ")
                        .and_then(|(sensor, beacon)| {
                            Some((sensor.split_once("Sensor at ").unwrap().1, beacon))
                        })
                })
                .map(|(sensor, beacon)| (parse_location(sensor), parse_location(beacon)))
                .flat_map(|(sensor_position, beacon_position)| {
                    [
                        (sensor_position, Tile::Sensor),
                        (beacon_position, Tile::Beacon),
                    ]
                })
                .collect(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(), 26);
    }

    #[test]
    fn test_map_from_str() {
        let map = Map::from("Sensor at x=2, y=18: closest beacon is at x=-2, y=15\nSensor at x=9, y=16: closest beacon is at x=10, y=16");
        assert_eq!(
            map,
            Map {
                tiles: HashMap::from([
                    ((2, 18), Tile::Sensor),
                    ((-2, 15), Tile::Beacon),
                    ((9, 16), Tile::Sensor),
                    ((10, 16), Tile::Beacon),
                ])
            }
        )
    }
}
