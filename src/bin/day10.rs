fn main() {
    let test_input = include_str!("../input/day10/test.txt");
    let prod_input = include_str!("../input/day10/prod.txt");

    let test_part1_result = part1(test_input);
    println!("test part1: {}", test_part1_result);
    let prod_part1_result = part1(prod_input);
    println!("prod part1: {}", prod_part1_result);

    let test_part2_result = part2(test_input);
    println!("test part2:\n{}", test_part2_result);
    let prod_part2_result = part2(prod_input);
    println!("prod part2:\n{}", prod_part2_result);
}

#[derive(Clone, Copy)]
enum Instruction {
    Noop,
    Addx(i32),
}

struct Program {
    cycle: u32,
    cycles_since_last_instruction: u32,
    register_x: i32,
    screen: String,
}

impl Program {
    fn new() -> Program {
        Program {
            cycle: 0,
            cycles_since_last_instruction: 0,
            register_x: 1,
            screen: String::new(),
        }
    }

    fn clock(&mut self) -> u32 {
        self.cycle += 1;
        self.cycles_since_last_instruction += 1;
        self.cycle
    }

    fn process_instruction(&mut self, instruction: Instruction) -> bool {
        match instruction {
            Instruction::Noop => {
                self.cycles_since_last_instruction = 0;
                true
            }
            Instruction::Addx(x) => {
                if self.cycles_since_last_instruction == 2 {
                    self.register_x += x;
                    self.cycles_since_last_instruction = 0;
                    return true;
                }
                false
            }
        }
    }

    fn draw(&mut self) {
        if (self.register_x - ((self.cycle - 1) % 40) as i32).abs() <= 1 {
            self.screen.push('#');
        } else {
            self.screen.push('.');
        }
        if self.cycle % 40 == 0 {
            self.screen.push('\n');
        }
    }

    fn get_strength(&self) -> i32 {
        self.register_x * self.cycle as i32
    }
}

fn part1(input: &str) -> i32 {
    let mut program = Program::new();

    let mut instructions = parse_instructions(input);

    let mut signal_strength = 0;
    let mut next_instruction = instructions.next();
    let cycles_to_track = [20, 60, 100, 140, 180, 220];
    while program.clock() <= 220 {
        if cycles_to_track.contains(&program.cycle) {
            signal_strength += program.get_strength();
        }
        match next_instruction {
            Some(instruction) => {
                if program.process_instruction(instruction) {
                    next_instruction = instructions.next();
                }
            }
            None => break,
        }
    }

    signal_strength
}

fn part2(input: &str) -> String {
    let mut program = Program::new();
    let mut instructions = parse_instructions(input);
    let mut next_instruction = instructions.next();
    while program.clock() <= 240 {
        program.draw();
        match next_instruction {
            Some(instruction) => {
                if program.process_instruction(instruction) {
                    next_instruction = instructions.next();
                }
            }
            None => break,
        }
    }

    program.screen
}

fn parse_instructions(input: &str) -> impl Iterator<Item = Instruction> + '_ {
    input.lines().map(|line| match line {
        "noop" => Instruction::Noop,
        addx_instruction_str => {
            let (_, x_num) = addx_instruction_str
                .split_once(' ')
                .expect("addx instruction should have space char");
            let x = x_num
                .parse::<i32>()
                .expect("addx instruction should have number");
            Instruction::Addx(x)
        }
    })
}
