fn main() {
    let input = include_str!("../input/day13/prod.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

fn part1(input: &str) -> u32 {
    let pairs = parse_input_part_1(input);
    pairs
        .iter()
        .enumerate()
        .filter_map(|(index, pair)| {
            if pair.is_in_right_order() {
                Some((index + 1) as u32)
            } else {
                None
            }
        })
        .sum()
}

fn part2(input: &str) -> u32 {
    let mut packets = parse_input_part_2(input);
    let divider2 = PacketLexer::new("[[2]]").parse();
    packets.push(divider2.clone());
    let divider6 = PacketLexer::new("[[6]]").parse();
    packets.push(divider6.clone());

    packets.sort();

    packets
        .iter()
        .enumerate()
        .filter_map(|(index, packet)| {
            if packet == &divider2 || packet == &divider6 {
                Some((index + 1) as u32)
            } else {
                None
            }
        })
        .product()
}

#[derive(Debug, Eq, Clone)]
enum Packet {
    Number(u32),
    List(Vec<Packet>),
}

impl PartialEq for Packet {
    fn eq(&self, other: &Self) -> bool {
        match (self, other) {
            (Self::Number(a), Self::Number(b)) => a == b,
            (Self::List(a), Self::List(b)) => a == b,
            (Self::Number(a), Self::List(b)) => &vec![Self::Number(*a)] == b,
            (Self::List(a), Self::Number(b)) => a == &vec![Self::Number(*b)],
        }
    }
}

impl Ord for Packet {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.partial_cmp(other).unwrap()
    }
}

impl PartialOrd for Packet {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match (self, other) {
            (Packet::Number(a), Packet::Number(b)) => a.partial_cmp(b),
            (Packet::List(a), Packet::List(b)) => a.partial_cmp(b),
            (Packet::Number(a), Packet::List(b)) => vec![Packet::Number(*a)].partial_cmp(b),
            (Packet::List(a), Packet::Number(b)) => a.partial_cmp(&vec![Packet::Number(*b)]),
        }
    }
}

#[derive(Debug)]
struct Pair {
    left: Packet,
    right: Packet,
}

impl Pair {
    fn is_in_right_order(&self) -> bool {
        match (&self.left, &self.right) {
            (Packet::List(a), Packet::List(b)) => a < b,
            _ => panic!("Pair should be constructed from two lists"),
        }
    }
}

#[derive(Debug)]
enum Token {
    OpenBracket,
    CloseBracket,
    Comma,
    Number(u32),
}

#[derive(Debug)]
struct PacketLexer {
    tokens: Vec<Token>,
}

impl PacketLexer {
    fn new(input: &str) -> Self {
        let mut tokens = Vec::new();
        let mut chars = input.chars().peekable();
        while let Some(c) = chars.next() {
            match c {
                ' ' => continue,
                '[' => tokens.push(Token::OpenBracket),
                ']' => tokens.push(Token::CloseBracket),
                ',' => tokens.push(Token::Comma),
                c if c.is_ascii_digit() => {
                    let mut number = String::new();
                    number.push(c);
                    while let Some(c) = chars.peek() {
                        if c.is_ascii_digit() {
                            number.push(*c);
                            chars.next();
                        } else {
                            break;
                        }
                    }
                    tokens.push(Token::Number(number.parse().unwrap()));
                }
                _ => panic!("invalid character"),
            }
        }
        Self { tokens }
    }

    fn parse(&mut self) -> Packet {
        let token = self.tokens.remove(0);
        match token {
            Token::OpenBracket => {
                let mut list = Vec::new();
                while let Some(token) = self.tokens.get(0) {
                    match token {
                        Token::CloseBracket => {
                            self.tokens.remove(0);
                            break;
                        }
                        Token::Comma => {
                            self.tokens.remove(0);
                        }
                        _ => list.push(self.parse()),
                    }
                }
                Packet::List(list)
            }
            Token::Number(n) => Packet::Number(n),
            _ => panic!("invalid token"),
        }
    }
}

fn parse_input_part_1(input: &str) -> Vec<Pair> {
    input
        .split("\n\n")
        .map(|pairs| {
            pairs
                .split_once('\n')
                .expect("pair should be separated by a newline")
        })
        .map(|(left, right)| Pair {
            left: PacketLexer::new(left).parse(),
            right: PacketLexer::new(right).parse(),
        })
        .collect()
}

fn parse_input_part_2(input: &str) -> Vec<Packet> {
    input
        .split("\n\n")
        .flat_map(|pairs| pairs.lines())
        .map(|line| PacketLexer::new(line).parse())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_1() {
        let input = include_str!("../input/day13/test.txt");
        assert_eq!(part1(input), 13);
    }

    #[test]
    fn test_part_2() {
        let input = include_str!("../input/day13/test.txt");
        assert_eq!(part2(input), 140);
    }
}
