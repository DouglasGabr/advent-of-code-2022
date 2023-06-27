use std::collections::HashMap;

use nom::{
    bytes::complete::tag,
    character::complete::{alpha1, digit1, one_of},
    combinator::map_res,
    sequence::{delimited, tuple},
};

fn main() {
    let input = include_str!("../input/day21.txt");
    println!("part 1: {}", part1(input));
}

enum MathOperation {
    Add,
    Subtract,
    Multiply,
    Divide,
}
impl From<char> for MathOperation {
    fn from(value: char) -> Self {
        match value {
            '+' => MathOperation::Add,
            '-' => MathOperation::Subtract,
            '*' => MathOperation::Multiply,
            '/' => MathOperation::Divide,
            _ => panic!("Invalid math operator"),
        }
    }
}
impl MathOperation {
    fn calculate(&self, lhs: i64, rhs: i64) -> i64 {
        match self {
            MathOperation::Add => lhs + rhs,
            MathOperation::Subtract => lhs - rhs,
            MathOperation::Multiply => lhs * rhs,
            MathOperation::Divide => lhs / rhs,
        }
    }
}

enum Job {
    YellNumber(i64),
    YellMathOperation(MathOperation, String, String),
}

struct Monkey {
    name: String,
    job: Job,
}

impl<'a> TryFrom<&'a str> for Monkey {
    type Error = nom::Err<nom::error::Error<&'a str>>;
    fn try_from(input: &'a str) -> Result<Self, Self::Error> {
        // root: pppw + sjmn
        // or
        // dbpl: 5
        let (input, monkey_name) = alpha1(input)?;
        let (input, _) = tag(": ")(input)?;
        let (_, job) = nom::branch::alt((
            map_res(digit1, |d: &str| {
                Ok::<_, Self::Error>(Job::YellNumber(d.parse::<i64>().unwrap()))
            }),
            map_res(
                tuple((
                    alpha1,
                    delimited(tag(" "), one_of("+-/*"), tag(" ")),
                    alpha1,
                )),
                |(monkey_name1, operator, monkey_name2): (&str, char, &str)| {
                    Ok::<_, Self::Error>(Job::YellMathOperation(
                        operator.into(),
                        monkey_name1.to_string(),
                        monkey_name2.to_string(),
                    ))
                },
            ),
        ))(input)?;
        Ok(Monkey {
            name: monkey_name.to_string(),
            job,
        })
    }
}

impl Monkey {
    fn yell(&self, monkeys: &HashMap<String, Monkey>) -> i64 {
        match &self.job {
            Job::YellNumber(num) => *num,
            Job::YellMathOperation(op, lhs, rhs) => {
                let lhs_result = monkeys
                    .get(lhs)
                    .expect("LHS monkey in hash map")
                    .yell(&monkeys);
                let rhs_result = monkeys
                    .get(rhs)
                    .expect("RHS monkey in hash map")
                    .yell(&monkeys);
                op.calculate(lhs_result, rhs_result)
            }
        }
    }
}

fn part1(input: &str) -> i64 {
    let monkeys = input
        .lines()
        .map(|line| Monkey::try_from(line).expect("Monkey to parse"))
        .map(|monkey| (monkey.name.clone(), monkey))
        .collect::<HashMap<_, _>>();
    let root_monkey = monkeys.get("root").expect("Root monkey in hash map");
    root_monkey.yell(&monkeys)
}

fn part2(input: &str) -> i64 {
    todo!()
}

#[cfg(test)]
mod test {
    use super::*;

    const TEST_INPUT: &'static str = "root: pppw + sjmn\n\
                                    dbpl: 5\n\
                                    cczh: sllz + lgvd\n\
                                    zczc: 2\n\
                                    ptdq: humn - dvpt\n\
                                    dvpt: 3\n\
                                    lfqf: 4\n\
                                    humn: 5\n\
                                    ljgn: 2\n\
                                    sjmn: drzm * dbpl\n\
                                    sllz: 4\n\
                                    pppw: cczh / lfqf\n\
                                    lgvd: ljgn * ptdq\n\
                                    drzm: hmdt - zczc\n\
                                    hmdt: 32";

    #[test]
    fn parse() {
        let monkeys = TEST_INPUT.lines().map(Monkey::try_from).collect::<Vec<_>>();
        assert!(monkeys.iter().all(|m| m.is_ok()));
    }

    #[test]
    fn test_part_1() {
        assert_eq!(part1(TEST_INPUT), 152);
    }

    #[test]
    fn test_part_2() {
        assert_eq!(part2(TEST_INPUT), 301);
    }
}
