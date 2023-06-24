use nom::{
    bytes::{complete::take_while, streaming::tag},
    character::is_digit,
    combinator::map_res,
    sequence::{preceded, separated_pair, terminated},
    IResult,
};

#[derive(Debug, PartialEq)]
enum ResourceCost {
    Ore(u32),
    Clay(u32),
    Obsidian(u32),
}

enum Robot {
    GeodeCracking,
    ObsidianCollecting,
    ClayCollecting,
    OreCollecting,
}

#[derive(Debug, PartialEq)]
struct Blueprint {
    ore_collecting_robot_cost: ResourceCost,
    clay_collecting_robot_cost: ResourceCost,
    obsidian_collecting_robot_cost: [ResourceCost; 2],
    geode_cracking_robot_cost: [ResourceCost; 2],
}

fn take_number(input: &str) -> IResult<&str, u32> {
    map_res(take_while(|c| is_digit(c as u8)), |num: &str| {
        num.parse::<u32>()
    })(input)
}

impl<'a> TryFrom<&'a str> for Blueprint {
    type Error = nom::Err<nom::error::Error<&'a str>>;

    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        let (input, _) = nom::sequence::tuple((tag("Blueprint "), take_number, tag(": ")))(input)?;
        let (input, ore_collecting_robot_cost) = map_res(
            preceded(
                tag("Each ore robot costs "),
                terminated(take_number, tag(" ore. ")),
            ),
            |cost| Ok::<_, Self::Error>(ResourceCost::Ore(cost)),
        )(input)?;
        let (input, clay_collecting_robot_cost) = map_res(
            preceded(
                tag("Each clay robot costs "),
                terminated(take_number, tag(" ore. ")),
            ),
            |cost| Ok::<_, Self::Error>(ResourceCost::Ore(cost)),
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
                    Ok::<_, Self::Error>([
                        ResourceCost::Ore(ore_cost),
                        ResourceCost::Clay(clay_cost),
                    ])
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
                |(ore_cost, clay_cost)| {
                    Ok::<_, Self::Error>([
                        ResourceCost::Ore(ore_cost),
                        ResourceCost::Obsidian(clay_cost),
                    ])
                },
            ),
        )(input)?;
        Ok(Blueprint {
            ore_collecting_robot_cost,
            clay_collecting_robot_cost,
            obsidian_collecting_robot_cost,
            geode_cracking_robot_cost,
        })
    }
}

fn main() {}

#[cfg(test)]
mod test {
    use super::*;
    #[test]
    fn blueprint_parser() {
        let blueprint = Blueprint::try_from("Blueprint 1: Each ore robot costs 3 ore. Each clay robot costs 4 ore. Each obsidian robot costs 2 ore and 20 clay. Each geode robot costs 4 ore and 7 obsidian.");
        assert_eq!(
            blueprint,
            Ok(Blueprint {
                ore_collecting_robot_cost: ResourceCost::Ore(3),
                clay_collecting_robot_cost: ResourceCost::Ore(4),
                obsidian_collecting_robot_cost: [ResourceCost::Ore(2), ResourceCost::Clay(20)],
                geode_cracking_robot_cost: [ResourceCost::Ore(4), ResourceCost::Obsidian(7)],
            })
        );
    }
}
