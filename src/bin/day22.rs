use std::{
    collections::HashMap,
    ops::{Add, Deref},
};

use nom::{
    branch::alt,
    character::complete::{digit1, one_of},
    combinator::map_res,
    multi::many0,
    IResult,
};

fn main() {
    let input = include_str!("../input/day22.txt");
    println!("part 1: {}", part1(input));
    println!("part 2: {}", part2(input));
}

fn part1(input: &str) -> u32 {
    let map = Map::from(input);
    let mut player = Player::new(map.find_starting_position());
    let instructions = Instructions::try_from(input).unwrap();
    for instruction in instructions.iter() {
        player.follow_instruction(&map, instruction);
    }
    1000 * player.position.0 + 4 * player.position.1 + player.facing.score()
}
fn part2(input: &str) -> usize {
    todo!()
}

#[derive(Debug)]
enum Tile {
    Open,
    Wall,
}

#[derive(Debug)]
struct Map {
    tiles: HashMap<(u32, u32), Tile>,
    width: u32,
    height: u32,
}

#[derive(Debug)]
enum TurnDirection {
    Clockwise,
    CounterClockwise,
}

#[derive(Debug)]
enum Instruction {
    Move(u32),
    Turn(TurnDirection),
}

#[derive(Debug)]
struct Instructions(Vec<Instruction>);
impl Deref for Instructions {
    type Target = Vec<Instruction>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl<'a> TryFrom<&'a str> for Instructions {
    type Error = nom::Err<nom::error::Error<&'a str>>;
    fn try_from(input: &'a str) -> Result<Instructions, Self::Error> {
        let instruction_str = input.split_once("\n\n").expect("double new-line").1;
        let (_, instructions) = many0(alt((
            map_res(one_of("RL"), |rl| {
                Ok::<Instruction, Self::Error>(match rl {
                    'R' => Instruction::Turn(TurnDirection::Clockwise),
                    'L' => Instruction::Turn(TurnDirection::CounterClockwise),
                    _ => unreachable!(),
                })
            }),
            map_res(digit1, |move_quantity: &str| {
                Ok::<Instruction, Self::Error>(Instruction::Move(
                    move_quantity.parse::<u32>().unwrap(),
                ))
            }),
        )))(instruction_str)?;
        Ok(Instructions(instructions))
    }
}

enum Direction {
    Right,
    Down,
    Left,
    Up,
}
impl Direction {
    fn score(&self) -> u32 {
        match self {
            Direction::Right => 0,
            Direction::Down => 1,
            Direction::Left => 2,
            Direction::Up => 3,
        }
    }
    fn turn(&mut self, turn_direction: &TurnDirection) {
        match (&self, turn_direction) {
            (Direction::Right, TurnDirection::Clockwise) => *self = Direction::Down,
            (Direction::Right, TurnDirection::CounterClockwise) => *self = Direction::Up,
            (Direction::Down, TurnDirection::Clockwise) => *self = Direction::Left,
            (Direction::Down, TurnDirection::CounterClockwise) => *self = Direction::Right,
            (Direction::Left, TurnDirection::Clockwise) => *self = Direction::Up,
            (Direction::Left, TurnDirection::CounterClockwise) => *self = Direction::Down,
            (Direction::Up, TurnDirection::Clockwise) => *self = Direction::Right,
            (Direction::Up, TurnDirection::CounterClockwise) => *self = Direction::Left,
        }
    }
}

struct Player {
    position: (u32, u32),
    facing: Direction,
}

impl Player {
    fn new(position: (u32, u32)) -> Self {
        Player {
            position,
            facing: Direction::Right,
        }
    }

    fn follow_instruction(&mut self, map: &Map, instruction: &Instruction) {
        match instruction {
            Instruction::Turn(turn_direction) => {
                self.facing.turn(turn_direction);
            }
            Instruction::Move(steps) => {
                for _ in 0..*steps {
                    let next_position = match self.facing {
                        Direction::Right => (self.position.0, self.position.1 + 1),
                        Direction::Down => (self.position.0 + 1, self.position.1),
                        Direction::Left => (self.position.0, self.position.1 - 1),
                        Direction::Up => (self.position.0 - 1, self.position.1),
                    };
                    let next_tile = map.tiles.get(&next_position);
                    match next_tile {
                        Some(Tile::Open) => self.position = next_position,
                        Some(Tile::Wall) => break,
                        None => {
                            let next_position = match self.facing {
                                Direction::Right => map.find_first_position_in_row(self.position.0),
                                Direction::Down => {
                                    map.find_first_position_in_column(self.position.1)
                                }
                                Direction::Left => map.find_last_position_in_row(self.position.0),
                                Direction::Up => map.find_last_position_in_column(self.position.1),
                            };
                            let next_tile = map.tiles.get(&next_position);
                            match next_tile {
                                Some(Tile::Open) => self.position = next_position,
                                Some(Tile::Wall) => break,
                                None => unreachable!(),
                            }
                        }
                    }
                }
            }
        }
    }
}

impl From<&str> for Map {
    fn from(value: &str) -> Self {
        let map_str = value.split_once("\n\n").expect("double new-line").0;
        let width = map_str.lines().map(|line| line.len()).max().expect("width");
        let height = map_str.lines().count();
        let mut tiles = HashMap::new();
        for row in 1..=height {
            for col in 1..=width {
                if let Some(tile) = match value
                    .lines()
                    .nth(row - 1)
                    .expect("row")
                    .chars()
                    .nth(col - 1)
                {
                    Some('.') => Some(Tile::Open),
                    Some('#') => Some(Tile::Wall),
                    _ => None,
                } {
                    tiles.insert((row as u32, col as u32), tile);
                }
            }
        }
        Map {
            tiles,
            width: width as u32,
            height: height as u32,
        }
    }
}

impl Map {
    fn find_starting_position(&self) -> (u32, u32) {
        self.tiles
            .iter()
            .filter(|((row, _), tile)| *row == 1 && matches!(tile, Tile::Open))
            .map(|(pos, _)| pos)
            .min_by_key(|(_, col)| col)
            .expect("starting position")
            .clone()
    }

    fn find_first_position_in_row(&self, row: u32) -> (u32, u32) {
        self.tiles
            .keys()
            .filter(|pos| pos.0 == row)
            .min_by_key(|(_, col)| col)
            .expect("first position in row")
            .clone()
    }
    fn find_last_position_in_row(&self, row: u32) -> (u32, u32) {
        self.tiles
            .keys()
            .filter(|pos| pos.0 == row)
            .max_by_key(|(_, col)| col)
            .expect("last position in row")
            .clone()
    }
    fn find_first_position_in_column(&self, column: u32) -> (u32, u32) {
        self.tiles
            .keys()
            .filter(|pos| pos.1 == column)
            .min_by_key(|(row, _)| row)
            .expect("first position in column")
            .clone()
    }
    fn find_last_position_in_column(&self, column: u32) -> (u32, u32) {
        self.tiles
            .keys()
            .filter(|pos| pos.1 == column)
            .max_by_key(|(row, _)| row)
            .expect("last position in column")
            .clone()
    }
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &'static str = "        ...#
        .#..
        #...
        ....
...#.......#
........#...
..#....#....
..........#.
        ...#....
        .....#..
        .#......
        ......#.

10R5L5R10L4R5L5";

    #[test]
    fn test_parse() {
        let map = Map::from(TEST_INPUT);
        println!("{:?}", map);
    }

    #[test]
    fn test_parse_instructions() {
        let instructions = Instructions::try_from(TEST_INPUT).expect("instructions");
        println!("{:?}", instructions);
    }

    #[test]
    fn test_first_starting_position() {
        let map = Map::from(TEST_INPUT);
        assert_eq!(map.find_starting_position(), (1, 9));
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part1(TEST_INPUT), 6032);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part2(TEST_INPUT), 0);
    }
}
