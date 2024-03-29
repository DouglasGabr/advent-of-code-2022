use std::{
    fmt::Display,
    ops::{Add, AddAssign, Sub, SubAssign},
};

use nom::{
    bytes::{complete::take_while, streaming::tag},
    character::is_digit,
    combinator::map_res,
    sequence::{preceded, separated_pair, terminated},
    IResult,
};

#[derive(Debug, PartialEq)]
enum Resource {
    Ore(u32),
    Clay(u32),
    Obsidian(u32),
    Geode(u32),
}

enum Robot {
    GeodeCracking,
    ObsidianCollecting,
    ClayCollecting,
    OreCollecting,
}
impl Display for Robot {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(match self {
            Robot::GeodeCracking => "geode-cracking",
            Robot::ObsidianCollecting => "obsidian-collecting",
            Robot::ClayCollecting => "clay-collecting",
            Robot::OreCollecting => "ore-collecting",
        })
    }
}

#[derive(Debug, PartialEq, Default, Copy, Clone, PartialOrd)]
struct Stock {
    ore: u32,
    clay: u32,
    obsidian: u32,
    geode: u32,
}

impl Add for Stock {
    type Output = Self;

    fn add(self, rhs: Self) -> Self::Output {
        Stock {
            ore: self.ore + rhs.ore,
            clay: self.clay + rhs.clay,
            obsidian: self.obsidian + rhs.obsidian,
            geode: self.geode + rhs.geode,
        }
    }
}

impl AddAssign for Stock {
    fn add_assign(&mut self, rhs: Self) {
        *self = Stock {
            ore: self.ore + rhs.ore,
            clay: self.clay + rhs.clay,
            obsidian: self.obsidian + rhs.obsidian,
            geode: self.geode + rhs.geode,
        }
    }
}

impl Add<&Robots> for Stock {
    type Output = Stock;

    fn add(self, rhs: &Robots) -> Self::Output {
        Stock {
            ore: self.ore + rhs.ore_collecting,
            clay: self.clay + rhs.clay_collecting,
            obsidian: self.obsidian + rhs.obsidian_collecting,
            geode: self.geode + rhs.geode_cracking,
        }
    }
}

impl Sub for Stock {
    type Output = Stock;

    fn sub(self, rhs: Self) -> Self::Output {
        Stock {
            ore: self.ore - rhs.ore,
            clay: self.clay - rhs.clay,
            obsidian: self.obsidian - rhs.obsidian,
            geode: self.geode - rhs.geode,
        }
    }
}

impl SubAssign for Stock {
    fn sub_assign(&mut self, rhs: Self) {
        self.ore -= rhs.ore;
        self.clay -= rhs.clay;
        self.obsidian -= rhs.obsidian;
        self.geode -= rhs.geode;
    }
}

impl Stock {
    fn new() -> Self {
        Stock {
            ore: 0,
            clay: 0,
            obsidian: 0,
            geode: 0,
        }
    }
    fn add(&mut self, resource: &Resource) {
        match resource {
            Resource::Ore(amount) => self.ore += amount,
            Resource::Clay(amount) => self.clay += amount,
            Resource::Obsidian(amount) => self.obsidian += amount,
            Resource::Geode(amount) => self.geode += amount,
        }
    }
}

struct Factory {
    stock: Stock,
    blueprint: Blueprint,
}
impl Factory {
    fn new(blueprint: Blueprint) -> Self {
        Factory {
            stock: Stock::new(),
            blueprint,
        }
    }

    fn create_robot(&mut self, robots: &Robots) -> Option<Robot> {
        let robot = self.blueprint.find_next_easiest_robot(&self.stock, robots);
        match robot {
            Some(Robot::GeodeCracking) if self.blueprint.can_create_geode_robot(&self.stock) => {
                self.stock -= self.blueprint.geode_cracking_robot_cost;
                println!(
                    "Spend {} ore and {} obsidian to start building a geode-cracking robot.",
                    self.blueprint.geode_cracking_robot_cost.ore,
                    self.blueprint.geode_cracking_robot_cost.obsidian
                );
                Some(Robot::GeodeCracking)
            }
            Some(Robot::ObsidianCollecting)
                if self.blueprint.can_create_obsidian_robot(&self.stock) =>
            {
                self.stock.ore -= self.blueprint.obsidian_collecting_robot_cost.ore;
                self.stock.clay -= self.blueprint.obsidian_collecting_robot_cost.clay;
                println!(
                    "Spend {} ore and {} clay to start building a obsidian-collecting robot.",
                    self.blueprint.obsidian_collecting_robot_cost.ore,
                    self.blueprint.obsidian_collecting_robot_cost.clay
                );
                Some(Robot::ObsidianCollecting)
            }
            Some(Robot::ClayCollecting) if self.blueprint.can_create_clay_robot(&self.stock) => {
                self.stock.ore -= self.blueprint.clay_collecting_robot_cost.ore;
                println!(
                    "Spend {} ore to start building a clay-collecting robot.",
                    self.blueprint.clay_collecting_robot_cost.ore
                );
                Some(Robot::ClayCollecting)
            }
            Some(Robot::OreCollecting) if self.blueprint.can_create_ore_robot(&self.stock) => {
                self.stock.ore -= self.blueprint.ore_collecting_robot_cost.ore;
                println!(
                    "Spend {} ore to start building a ore-collecting robot.",
                    self.blueprint.ore_collecting_robot_cost.ore
                );
                Some(Robot::OreCollecting)
            }
            _ => None,
        }
    }
}

#[derive(Debug, PartialEq)]
struct Blueprint {
    id: u32,
    ore_collecting_robot_cost: Stock,
    clay_collecting_robot_cost: Stock,
    obsidian_collecting_robot_cost: Stock,
    geode_cracking_robot_cost: Stock,
    max_required_robots: Stock,
}
impl Blueprint {
    fn find_next_easiest_robot(&self, stock: &Stock, robots: &Robots) -> Option<Robot> {
        [
            (
                Robot::OreCollecting,
                (self.max_required_robots.ore > robots.ore_collecting)
                    .then(|| {
                        self.ore_collecting_robot_cost
                            .ore
                            .checked_sub(stock.ore)
                            .unwrap_or(0)
                            .checked_div(robots.ore_collecting)
                            .and_then(|time| Some(time * 4))
                    })
                    .flatten(),
            ),
            (
                Robot::ClayCollecting,
                (self.max_required_robots.clay > robots.clay_collecting)
                    .then(|| {
                        self.clay_collecting_robot_cost
                            .ore
                            .checked_sub(stock.ore)
                            .unwrap_or(0)
                            .checked_div(robots.ore_collecting)
                            .and_then(|time| Some(time * 3))
                    })
                    .flatten(),
            ),
            (
                Robot::ObsidianCollecting,
                (self.max_required_robots.obsidian > robots.obsidian_collecting)
                    .then(|| {
                        self.obsidian_collecting_robot_cost
                            .ore
                            .checked_sub(stock.ore)
                            .unwrap_or(0)
                            .checked_div(robots.ore_collecting)
                            .max(
                                self.obsidian_collecting_robot_cost
                                    .clay
                                    .checked_sub(stock.clay)
                                    .unwrap_or(0)
                                    .checked_div(robots.clay_collecting),
                            )
                            .and_then(|time| Some(time * 2))
                    })
                    .flatten(),
            ),
            (
                Robot::GeodeCracking,
                self.geode_cracking_robot_cost
                    .ore
                    .checked_sub(stock.ore)
                    .unwrap_or(0)
                    .checked_div(robots.ore_collecting)
                    .max(
                        self.geode_cracking_robot_cost
                            .obsidian
                            .checked_sub(stock.obsidian)
                            .unwrap_or(0)
                            .checked_div(robots.obsidian_collecting),
                    ),
            ),
        ]
        .into_iter()
        .min_by_key(|(_, time)| *time)
        .map(|(robot, _)| robot)
    }

    fn can_create_geode_robot(&self, stock: &Stock) -> bool {
        stock >= &self.geode_cracking_robot_cost
    }
    fn can_create_obsidian_robot(&self, stock: &Stock) -> bool {
        stock >= &self.obsidian_collecting_robot_cost
    }
    fn can_create_clay_robot(&self, stock: &Stock) -> bool {
        stock >= &self.clay_collecting_robot_cost
    }
    fn can_create_ore_robot(&self, stock: &Stock) -> bool {
        stock >= &self.ore_collecting_robot_cost
    }
}

fn take_number(input: &str) -> IResult<&str, u32> {
    map_res(take_while(|c| is_digit(c as u8)), |num: &str| {
        num.parse::<u32>()
    })(input)
}

struct Robots {
    geode_cracking: u32,
    obsidian_collecting: u32,
    clay_collecting: u32,
    ore_collecting: u32,
}
impl Robots {
    fn new() -> Self {
        Robots {
            geode_cracking: 0,
            obsidian_collecting: 0,
            clay_collecting: 0,
            ore_collecting: 1,
        }
    }
    fn add_robot(&mut self, robot: &Robot) {
        match robot {
            Robot::GeodeCracking => self.geode_cracking += 1,
            Robot::ObsidianCollecting => self.obsidian_collecting += 1,
            Robot::ClayCollecting => self.clay_collecting += 1,
            Robot::OreCollecting => self.ore_collecting += 1,
        }
        println!(
            "The new {} robot is ready; you now have {} of them.",
            robot,
            match robot {
                Robot::GeodeCracking => self.geode_cracking,
                Robot::ObsidianCollecting => self.obsidian_collecting,
                Robot::ClayCollecting => self.clay_collecting,
                Robot::OreCollecting => self.ore_collecting,
            }
        );
    }
    fn mine(&self, stock: &mut Stock) {
        let mined = Stock {
            ore: self.ore_collecting,
            clay: self.clay_collecting,
            obsidian: self.obsidian_collecting,
            geode: self.geode_cracking,
        };
        *stock += mined;
        if mined.ore > 0 {
            println!(
                "{} ore-collecting robot collect {} ore; you now have {} ore.",
                self.ore_collecting, mined.ore, stock.ore
            );
        }
        if mined.clay > 0 {
            println!(
                "{} clay-collecting robot collect {} clay; you now have {} clay.",
                self.clay_collecting, mined.clay, stock.clay
            );
        }
        if mined.obsidian > 0 {
            println!(
                "{} obsidian-collecting robot collect {} obsidian; you now have {} obsidian.",
                self.obsidian_collecting, mined.obsidian, stock.obsidian
            );
        }
        if mined.geode > 0 {
            println!(
                "{} geode-cracking robot crack {} geode; you now have {} open geodes.",
                self.geode_cracking, mined.geode, stock.geode
            );
        }
    }
}

impl<'a> TryFrom<&'a str> for Blueprint {
    type Error = nom::Err<nom::error::Error<&'a str>>;

    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        let (input, id) =
            nom::sequence::delimited(tag("Blueprint "), take_number, tag(": "))(input)?;
        let (input, ore_collecting_robot_cost) = map_res(
            preceded(
                tag("Each ore robot costs "),
                terminated(take_number, tag(" ore. ")),
            ),
            |cost| {
                Ok::<_, Self::Error>(Stock {
                    ore: cost,
                    ..Default::default()
                })
            },
        )(input)?;
        let (input, clay_collecting_robot_cost) = map_res(
            preceded(
                tag("Each clay robot costs "),
                terminated(take_number, tag(" ore. ")),
            ),
            |cost| {
                Ok::<_, Self::Error>(Stock {
                    ore: cost,
                    ..Default::default()
                })
            },
        )(input)?;
        let (input, obsidian_collecting_robot_cost) = preceded(
            tag("Each obsidian robot costs "),
            map_res(
                separated_pair(
                    terminated(take_number, tag(" ore")),
                    tag(" and "),
                    terminated(take_number, tag(" clay. ")),
                ),
                |(ore_cost, clay_cost)| {
                    Ok::<_, Self::Error>(Stock {
                        ore: ore_cost,
                        clay: clay_cost,
                        ..Default::default()
                    })
                },
            ),
        )(input)?;
        let (_, geode_cracking_robot_cost) = preceded(
            tag("Each geode robot costs "),
            map_res(
                separated_pair(
                    terminated(take_number, tag(" ore")),
                    tag(" and "),
                    terminated(take_number, tag(" obsidian.")),
                ),
                |(ore_cost, obsidian_cost)| {
                    Ok::<_, Self::Error>(Stock {
                        ore: ore_cost,
                        obsidian: obsidian_cost,
                        ..Default::default()
                    })
                },
            ),
        )(input)?;
        Ok(Blueprint {
            id,
            ore_collecting_robot_cost,
            clay_collecting_robot_cost,
            obsidian_collecting_robot_cost,
            geode_cracking_robot_cost,
            max_required_robots: Stock {
                geode: u32::MAX,
                obsidian: geode_cracking_robot_cost.obsidian,
                clay: obsidian_collecting_robot_cost.clay,
                ore: ore_collecting_robot_cost
                    .ore
                    .max(clay_collecting_robot_cost.ore)
                    .max(obsidian_collecting_robot_cost.ore)
                    .max(geode_cracking_robot_cost.ore),
            },
        })
    }
}

fn main() {}

fn part1(input: &str) -> u32 {
    input
        .lines()
        .flat_map(|line| Blueprint::try_from(line))
        .map(|blueprint| {
            let mut factory = Factory::new(blueprint);
            let mut robots = Robots::new();
            for minute in 1..=24 {
                println!("== Minute {minute} ==");
                let new_robot = factory.create_robot(&robots);
                robots.mine(&mut factory.stock);
                match new_robot {
                    Some(robot) => robots.add_robot(&robot),
                    None => {}
                }
                println!("");
            }
            factory.blueprint.id * factory.stock.geode
        })
        .sum()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn blueprint_parser() {
        let blueprint = Blueprint::try_from("Blueprint 1: Each ore robot costs 3 ore. Each clay robot costs 4 ore. Each obsidian robot costs 2 ore and 20 clay. Each geode robot costs 4 ore and 7 obsidian.");
        assert_eq!(
            blueprint,
            Ok(Blueprint {
                id: 1,
                ore_collecting_robot_cost: Stock {
                    ore: 3,
                    ..Default::default()
                },
                clay_collecting_robot_cost: Stock {
                    ore: 4,
                    ..Default::default()
                },
                obsidian_collecting_robot_cost: Stock {
                    ore: 2,
                    clay: 20,
                    ..Default::default()
                },
                geode_cracking_robot_cost: Stock {
                    ore: 4,
                    obsidian: 7,
                    ..Default::default()
                },
                max_required_robots: Stock {
                    ore: 4,
                    clay: 20,
                    obsidian: 7,
                    geode: u32::MAX
                }
            })
        );
    }

    #[test]
    fn part1() {
        let input = "Blueprint 1: \
                        Each ore robot costs 4 ore. \
                        Each clay robot costs 2 ore. \
                        Each obsidian robot costs 3 ore and 14 clay. \
                        Each geode robot costs 2 ore and 7 obsidian.\n\
                    Blueprint 2: \
                        Each ore robot costs 2 ore. \
                        Each clay robot costs 3 ore. \
                        Each obsidian robot costs 3 ore and 8 clay. \
                        Each geode robot costs 3 ore and 12 obsidian.";
        println!("{:?}", super::part1(input));
    }
}
