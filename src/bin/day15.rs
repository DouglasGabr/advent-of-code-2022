use std::{
    collections::{HashMap, HashSet},
    ops::RangeBounds,
};

use ranges::Ranges;

fn main() {
    let input = include_str!("../input/day15/prod.txt");
    println!("Part 1: {}", part1(input, 2_000_000));
    println!("Part 2: {}", part2(input, 4_000_000));
}

fn part1(input: &str, row_to_check: i32) -> u32 {
    let map = Map::from(input);
    let mut scanned_positions = map
        .sensor_area
        .iter()
        .filter(|(sensor_position, range)| (sensor_position.1 - row_to_check).abs() <= **range)
        .flat_map(|(valid_sensor, range)| {
            let distance = (valid_sensor.1 - row_to_check).abs();
            let spread = range - distance;
            let left = valid_sensor.0 - spread;
            let right = valid_sensor.0 + spread;
            (left..=right).map(|x| (x, row_to_check))
        })
        .collect::<HashSet<_>>();
    for beacon in map.known_beacons {
        scanned_positions.remove(&beacon);
    }
    scanned_positions.len() as u32
}

fn part2(input: &str, limit: i32) -> u64 {
    let map = Map::from(input);
    let mut ranges: Vec<Ranges<i32>> = (0..=limit).map(|_| Ranges::new()).collect();
    map.sensor_area.iter().for_each(|(position, range)| {
        ((position.1 - range).max(0)..=(position.1 + range).min(limit)).for_each(|y| {
            let distance = (position.1 - y).abs();
            let spread = range - distance;
            ranges[y as usize]
                .insert((position.0 - spread).max(0)..=(position.0 + spread).min(limit));
        })
    });
    let distress_beacon_position = (0..=limit)
        .find_map(|y| {
            let range = ranges[y as usize].clone();
            let diff = Ranges::from(0..=limit) - range;
            if !diff.is_empty() {
                let start_bound = diff.as_slice().first().unwrap().start_bound();
                match start_bound {
                    std::ops::Bound::Included(x) => Some((*x, y)),
                    std::ops::Bound::Excluded(x) => Some((*x + 1, y)),
                    _ => panic!("unexpected bound"),
                }
            } else {
                None
            }
        })
        .unwrap();
    distress_beacon_position.0 as u64 * 4_000_000 + distress_beacon_position.1 as u64
}

#[derive(PartialEq, Debug)]
struct Map {
    sensor_area: HashMap<(i32, i32), i32>,
    known_beacons: HashSet<(i32, i32)>,
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
        let mut map = Map {
            sensor_area: HashMap::new(),
            known_beacons: HashSet::new(),
        };
        for (sensor_position, beacon_position) in value
            .lines()
            .filter_map(|line| {
                line.split_once(": closest beacon is at ")
                    .and_then(|(sensor, beacon)| {
                        Some((sensor.split_once("Sensor at ").unwrap().1, beacon))
                    })
            })
            .map(|(sensor, beacon)| (parse_location(sensor), parse_location(beacon)))
            .map(|(sensor_position, beacon_position)| (sensor_position, beacon_position))
        {
            let distance = (sensor_position.0 - beacon_position.0).abs()
                + (sensor_position.1 - beacon_position.1).abs();
            map.sensor_area.insert(sensor_position, distance);
            map.known_beacons.insert(beacon_position);
        }
        map
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../input/day15/test.txt");
        assert_eq!(part1(input, 10), 26);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("../input/day15/test.txt");
        assert_eq!(part2(input, 20), 56000011);
    }

    #[test]
    fn test_map_from_str() {
        let map = Map::from("Sensor at x=2, y=18: closest beacon is at x=-2, y=15\nSensor at x=9, y=16: closest beacon is at x=10, y=16");
        assert_eq!(
            map,
            Map {
                sensor_area: HashMap::from([((2, 18), 7), ((9, 16), 1),]),
                known_beacons: HashSet::from([(10, 16), (-2, 15),]),
            }
        )
    }
}
