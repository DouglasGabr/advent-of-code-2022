use std::{
    collections::HashMap,
    hash::{Hash, Hasher},
};

use petgraph::{
    algo::dijkstra,
    data::FromElements,
    dot::{Config, Dot},
    graph::DiGraph,
    prelude::DiGraphMap,
};

fn main() {}

fn part1(input: &str) -> u32 {
    let graph = parse_input(input);
    todo!()
}

fn part2(input: &str) -> u32 {
    todo!()
}

#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd, Eq, Ord)]
struct Valve<'a> {
    name: &'a str,
    flow_rate: u32,
}

impl<'a> Valve<'a> {
    fn from_name(name: &'a str) -> Self {
        Self { name, flow_rate: 0 }
    }
}

impl Hash for Valve<'_> {
    fn hash<H: Hasher>(&self, state: &mut H) {
        self.name.hash(state);
    }
}

fn parse_input(input: &str) -> DiGraphMap<Valve, ()> {
    let map: HashMap<&str, (Valve, Vec<&str>)> = input
        .lines()
        .map(|line| line.split_once("; ").unwrap())
        .map(|(valve_info, tunnels_info)| {
            (
                valve_info.split_whitespace().collect::<Vec<&str>>(),
                tunnels_info
                    .split_once("valves ")
                    .or_else(|| tunnels_info.split_once("valve "))
                    .unwrap()
                    .1,
            )
        })
        .map(|(valve_info_pieces, other_valves)| {
            (
                valve_info_pieces[1],
                (
                    Valve {
                        name: valve_info_pieces[1],
                        flow_rate: valve_info_pieces[4]
                            .split_once('=')
                            .unwrap()
                            .1
                            .parse()
                            .unwrap(),
                    },
                    other_valves.split(", ").collect(),
                ),
            )
        })
        .collect();

    DiGraphMap::from_edges(map.values().flat_map(|(valve, other_valves)| {
        other_valves
            .iter()
            .map(|other_valve| (valve.clone(), map[other_valve].0.clone()))
    }))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("../input/day16/test.txt");
        assert_eq!(part1(input), 1651);
    }

    #[test]
    #[ignore]
    fn test_part2() {
        let input = include_str!("../input/day16/test.txt");
        assert_eq!(part2(input), 0);
    }
}
