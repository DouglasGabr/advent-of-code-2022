use std::{
    collections::HashSet,
    ops::{Add, Sub},
};

enum Movement {
    Right(i32),
    Up(i32),
    Left(i32),
    Down(i32),
}

fn main() {
    let test_input = include_str!("../input/day9/test.txt");
    let prod_input = include_str!("../input/day9/prod.txt");

    const PART_1_ROPE_SIZE: usize = 2;
    let test_part1_result = run::<PART_1_ROPE_SIZE>(test_input);
    println!("Test Part 1: {}", test_part1_result);
    let prod_part1_result = run::<PART_1_ROPE_SIZE>(prod_input);
    println!("Prod Part 1: {}", prod_part1_result);

    const PART_2_ROPE_SIZE: usize = 10;
    let test_part2_result = run::<PART_2_ROPE_SIZE>(test_input);
    println!("Test Part 2: {}", test_part2_result);
    let prod_part2_result = run::<PART_2_ROPE_SIZE>(prod_input);
    println!("Prod Part 2: {}", prod_part2_result);
}

#[derive(PartialEq, Eq, Hash, Debug, PartialOrd, Clone, Copy)]
struct Coordinate(i32, i32);

#[derive(Debug, PartialEq, PartialOrd)]
struct Distance(i32, i32);

impl Sub for Coordinate {
    type Output = Distance;
    fn sub(self, other: Coordinate) -> Distance {
        Distance(self.0 - other.0, self.1 - other.1)
    }
}

impl Add for Coordinate {
    type Output = Coordinate;
    fn add(self, other: Coordinate) -> Coordinate {
        Coordinate(self.0 + other.0, self.1 + other.1)
    }
}

struct Rope<const SIZE: usize> {
    body: [Coordinate; SIZE],
    tail_visited_coordinates: HashSet<Coordinate>,
}

impl<const SIZE: usize> Rope<SIZE> {
    fn new() -> Self {
        let mut tail_visited_coordinates = HashSet::new();
        tail_visited_coordinates.insert(Coordinate(0, 0));
        Rope {
            body: [Coordinate(0, 0); SIZE],
            tail_visited_coordinates,
        }
    }

    fn apply_movement(&mut self, movement: Movement) {
        match movement {
            Movement::Right(distance) => self.move_head(Coordinate(1, 0), distance),
            Movement::Up(distance) => self.move_head(Coordinate(0, 1), distance),
            Movement::Left(distance) => self.move_head(Coordinate(-1, 0), distance),
            Movement::Down(distance) => self.move_head(Coordinate(0, -1), distance),
        }
    }

    fn move_head(&mut self, direction_coordinate: Coordinate, distance: i32) {
        for _ in 0..distance {
            let current_head = self.body[0];
            let new_head = current_head + direction_coordinate;
            self.body[0] = new_head;
            for i in 1..SIZE {
                let current_knot = self.body[i];
                let previous_knot = self.body[i - 1];
                let distance = previous_knot - current_knot;
                if distance.0.abs() > 1 || distance.1.abs() > 1 {
                    let new_knot_position =
                        current_knot + Coordinate(distance.0.clamp(-1, 1), distance.1.clamp(-1, 1));
                    self.body[i] = new_knot_position;
                }
            }
            self.tail_visited_coordinates.insert(self.body[SIZE - 1]);
        }
    }
}

fn run<const ROPE_SIZE: usize>(input: &str) -> i32 {
    let mut rope = Rope::<ROPE_SIZE>::new();

    input
        .lines()
        .flat_map(|line| line.split_once(' '))
        .map(|(direction, distance)| {
            let distance = distance.parse::<i32>().expect("distance should be an i32");
            match direction {
                "R" => Movement::Right(distance),
                "U" => Movement::Up(distance),
                "L" => Movement::Left(distance),
                "D" => Movement::Down(distance),
                _ => panic!("unknown direction"),
            }
        })
        .for_each(|movement| rope.apply_movement(movement));

    rope.tail_visited_coordinates
        .len()
        .try_into()
        .expect("visited coordinates should have a length that can be converted to an i32")
}
