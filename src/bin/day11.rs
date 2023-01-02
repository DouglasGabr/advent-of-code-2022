fn main() {
    let test_input = include_str!("../input/day11/test.txt");
    let prod_input = include_str!("../input/day11/prod.txt");

    let part1_test_result = part1(test_input);
    println!("Part 1 test result: {}", part1_test_result);

    let part1_prod_result = part1(prod_input);
    println!("Part 1 prod result: {}", part1_prod_result);

    let part2_test_result = part2(test_input);
    println!("Part 2 test result: {}", part2_test_result);

    let part2_prod_result = part2(prod_input);
    println!("Part 2 prod result: {}", part2_prod_result);
}

fn part1(input: &str) -> u64 {
    let mut monkeys = parse_input(input);

    let rounds = 20;
    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            let inspection_results = monkeys[i].inspect_items(|item| item.0 /= 3);
            for inspection_result in inspection_results {
                monkeys[inspection_result.new_monkey_index]
                    .items
                    .push(inspection_result.item);
            }
        }
    }

    monkeys.sort_by(|a, b| b.inspections_count.cmp(&a.inspections_count));

    monkeys
        .iter()
        .take(2)
        .map(|monkey| monkey.inspections_count)
        .product()
}

fn part2(input: &str) -> u64 {
    let mut monkeys = parse_input(input);

    let lcm = monkeys
        .iter()
        .map(|monkey| monkey.test_divisible_by)
        .reduce(|a, b| a * b)
        .expect("at least one monkey");

    let rounds = 10_000;
    for _ in 0..rounds {
        for i in 0..monkeys.len() {
            let inspection_results = monkeys[i].inspect_items(|item| item.0 %= lcm);
            for inspection_result in inspection_results {
                monkeys[inspection_result.new_monkey_index]
                    .items
                    .push(inspection_result.item);
            }
        }
    }

    monkeys.sort_by(|a, b| b.inspections_count.cmp(&a.inspections_count));

    monkeys
        .iter()
        .take(2)
        .map(|monkey| monkey.inspections_count)
        .product()
}

#[derive(Clone, Copy, PartialEq)]
enum Operator {
    Add,
    Multiply,
}

impl From<&str> for Operator {
    fn from(value: &str) -> Self {
        match value {
            "+" => Operator::Add,
            "*" => Operator::Multiply,
            _ => panic!("Unknown operator: {}", value),
        }
    }
}

#[derive(Clone, Copy)]
enum OperationItem {
    ItemLevel,
    Literal(u64),
}

impl From<&str> for OperationItem {
    fn from(value: &str) -> Self {
        match value {
            "old" => OperationItem::ItemLevel,
            num_str => OperationItem::Literal(
                num_str
                    .parse()
                    .expect("value should be either 'old' or a number"),
            ),
        }
    }
}

#[derive(Clone, Copy)]
struct Operation {
    lhs: OperationItem,
    operator: Operator,
    rhs: OperationItem,
}

fn apply_operation_on_item(item: &mut Item, operation: Operation) -> () {
    let lhs = match operation.lhs {
        OperationItem::ItemLevel => item.0,
        OperationItem::Literal(literal) => literal,
    };
    let rhs = match operation.rhs {
        OperationItem::ItemLevel => item.0,
        OperationItem::Literal(literal) => literal,
    };
    match operation.operator {
        Operator::Add => item.0 = lhs + rhs,
        Operator::Multiply => item.0 = lhs * rhs,
    }
}

#[derive(Clone, Copy)]
struct Item(u64);

struct InspectionResult {
    new_monkey_index: usize,
    item: Item,
}

struct Monkey {
    items: Vec<Item>,
    operation: Operation,
    test_divisible_by: u64,
    destination_monkey_indexes: (usize, usize),
    inspections_count: u64,
}

impl Monkey {
    fn inspect_items<F: FnMut(&mut Item) -> ()>(
        &mut self,
        mut after_inspection: F,
    ) -> Vec<InspectionResult> {
        let result: Vec<InspectionResult> = self
            .items
            .iter_mut()
            .map(|item| {
                apply_operation_on_item(item, self.operation);
                item
            })
            .map(|item| {
                after_inspection(item);
                item
            })
            .map(|item| match item.0 % self.test_divisible_by {
                0 => InspectionResult {
                    new_monkey_index: self.destination_monkey_indexes.1,
                    item: *item,
                },
                _ => InspectionResult {
                    new_monkey_index: self.destination_monkey_indexes.0,
                    item: *item,
                },
            })
            .collect();

        self.items.clear();

        self.inspections_count += result.len() as u64;

        result
    }
}

#[derive(Default)]
struct MonkeyBuilder {
    items: Vec<Item>,
    operation: Option<Operation>,
    test_divisible_by: Option<u64>,
    destination_monkey_index_if_true: Option<usize>,
    destination_monkey_index_if_false: Option<usize>,
}

impl MonkeyBuilder {
    fn new() -> Self {
        MonkeyBuilder {
            items: Vec::new(),
            ..Default::default()
        }
    }
    fn add_item(&mut self, item: Item) -> &mut Self {
        self.items.push(item);
        self
    }

    fn set_operation(&mut self, operation: Operation) -> &mut Self {
        self.operation = Some(operation);
        self
    }

    fn set_test_divisible_by(&mut self, test_divisible_by: u64) -> &mut Self {
        self.test_divisible_by = Some(test_divisible_by);
        self
    }

    fn set_destination_monkey_index_if_true(
        &mut self,
        destination_monkey_index: usize,
    ) -> &mut Self {
        self.destination_monkey_index_if_true = Some(destination_monkey_index);
        self
    }
    fn set_destination_monkey_index_if_false(
        &mut self,
        destination_monkey_index: usize,
    ) -> &mut Self {
        self.destination_monkey_index_if_false = Some(destination_monkey_index);
        self
    }

    fn build(self) -> Result<Monkey, &'static str> {
        Ok(Monkey {
            items: self.items,
            operation: self.operation.ok_or("operation not set")?,
            test_divisible_by: self.test_divisible_by.ok_or("test_divisible_by no set")?,
            destination_monkey_indexes: (
                self.destination_monkey_index_if_false
                    .ok_or("destination_monkey_index_if_false not set")?,
                self.destination_monkey_index_if_true
                    .ok_or("destination_monkey_index_if_true not set")?,
            ),
            inspections_count: 0,
        })
    }
}

fn parse_input(input: &str) -> Vec<Monkey> {
    input
        .split("\n\n")
        .map(|monkey_details| {
            let mut monkey_builder = MonkeyBuilder::new();
            for detail_line in monkey_details.lines().skip(1) {
                if detail_line.trim().starts_with("Starting items:") {
                    detail_line
                        .split_once(": ")
                        .map(|(_, item_worry_levels)| item_worry_levels)
                        .map(|item_worry_levels| item_worry_levels.split(", "))
                        .map(|item_worry_levels| {
                            item_worry_levels
                                .map(|item_worry_level| Item(item_worry_level.parse().unwrap()))
                        })
                        .unwrap()
                        .for_each(|item| {
                            monkey_builder.add_item(item);
                        });
                } else if detail_line.trim().starts_with("Operation:") {
                    let operation = detail_line
                        .split_once("new = ")
                        .map(|(_, operation)| operation)
                        .map(|operation| {
                            operation.split_whitespace().enumerate().fold(
                                [None; 3],
                                |mut acc, (index, piece)| {
                                    acc[index] = Some(piece);
                                    acc
                                },
                            )
                        })
                        .map(|arr| match arr {
                            [Some(lhs), Some(op), Some(rhs)] => Operation {
                                lhs: lhs.into(),
                                operator: op.into(),
                                rhs: rhs.into(),
                            },
                            _ => panic!("invalid operation"),
                        })
                        .expect("valid operation");
                    monkey_builder.set_operation(operation);
                } else if detail_line.trim().starts_with("Test:") {
                    let test_divisible_by = detail_line
                        .split_once("divisible by ")
                        .map(|(_, test_divisible_by)| test_divisible_by)
                        .map(|test_divisible_by| test_divisible_by.parse().unwrap())
                        .expect("valid test_divisible_by");
                    monkey_builder.set_test_divisible_by(test_divisible_by);
                } else if detail_line.trim().starts_with("If true:") {
                    let destination_monkey_index = detail_line
                        .split_once("throw to monkey ")
                        .map(|(_, destination_monkey_index)| destination_monkey_index)
                        .map(|destination_monkey_index| destination_monkey_index.parse().unwrap())
                        .expect("valid destination_monkey_index");
                    monkey_builder.set_destination_monkey_index_if_true(destination_monkey_index);
                } else if detail_line.trim().starts_with("If false:") {
                    let destination_monkey_index = detail_line
                        .split_once("throw to monkey ")
                        .map(|(_, destination_monkey_index)| destination_monkey_index)
                        .map(|destination_monkey_index| destination_monkey_index.parse().unwrap())
                        .expect("valid destination_monkey_index");
                    monkey_builder.set_destination_monkey_index_if_false(destination_monkey_index);
                }
            }
            monkey_builder.build().unwrap()
        })
        .collect()
}
