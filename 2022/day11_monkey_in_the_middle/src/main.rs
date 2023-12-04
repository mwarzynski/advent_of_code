use std::collections::VecDeque;
use std::fs::File as FSFile;
use std::io::{BufRead, BufReader};
use std::str::FromStr;

#[derive(Debug)]
struct Item {
    worry_level: i64,
}

impl Item {
    fn new(worry_level: i64) -> Self {
        Self { worry_level }
    }
}

impl FromStr for Item {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s.parse::<i64>() {
            Ok(worry_level) => Ok(Self { worry_level }),
            Err(_) => Err(()),
        }
    }
}

#[derive(Debug)]
enum Value {
    Old(),
    Value(i64),
}

#[derive(Debug)]
enum Operation {
    Add(),
    Multiply(),
}

#[derive(Debug)]
struct MonkeyOperation {
    v1: Value,
    op: Operation,
    v2: Value,
}

impl MonkeyOperation {
    fn perform(&self, old: i64) -> i64 {
        let v1 = match self.v1 {
            Value::Old() => old,
            Value::Value(v) => v,
        };
        let v2 = match self.v2 {
            Value::Old() => old,
            Value::Value(v) => v,
        };
        match self.op {
            Operation::Add() => v1 + v2,
            Operation::Multiply() => v1 * v2,
        }
    }
}

#[derive(Debug)]
struct MonkeyTest {
    divisible_by: i64,
    if_true_throw_to_monkey: usize,
    if_false_throw_to_monkey: usize,
}

impl MonkeyTest {
    fn perform(&self, worry_level: i64) -> usize {
        if worry_level % self.divisible_by == 0 {
            self.if_true_throw_to_monkey
        } else {
            self.if_false_throw_to_monkey
        }
    }
}

#[derive(Debug)]
struct Monkey {
    items: VecDeque<Item>,
    operation: MonkeyOperation,
    test: MonkeyTest,

    items_inspections: u32,
}

impl Monkey {
    fn has_items(&self) -> bool {
        !self.items.is_empty()
    }

    fn throw_item(&mut self) -> (usize, Item) {
        let item = self.items.pop_front().unwrap();

        // perform operation
        let mut worry_level = item.worry_level;
        worry_level = self.operation.perform(worry_level);
        // worry_level = worry_level / 3; // monkey gets bored (part1 only)
        worry_level = worry_level % 9699690; // part2, to avoid overflow

        // perform test
        let monkey_receiver = self.test.perform(worry_level);

        self.items_inspections += 1;
        (monkey_receiver, Item::new(worry_level))
    }

    fn receive_item(&mut self, item: Item) {
        self.items.push_back(item)
    }
}

impl FromStr for Monkey {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut items = VecDeque::new();
        let mut operation: Option<MonkeyOperation> = None;
        let mut test: MonkeyTest = MonkeyTest {
            divisible_by: 0,
            if_true_throw_to_monkey: 0,
            if_false_throw_to_monkey: 0,
        };

        for line in s.lines() {
            if line.starts_with("  Starting items: ") {
                line.strip_prefix("  Starting items: ")
                    .unwrap()
                    .split(", ")
                    .for_each(|item| items.push_back(item.parse().unwrap()));
            }
            if line.starts_with("  Operation: new = ") {
                let operation_str = line.strip_prefix("  Operation: new = ").unwrap();
                let ops: Vec<&str> = operation_str.split(" ").collect();
                let v1 = match ops[0] {
                    "old" => Value::Old(),
                    _ => Value::Value(ops[0].parse().unwrap()),
                };
                let op = match ops[1] {
                    "+" => Operation::Add(),
                    "*" => Operation::Multiply(),
                    _ => panic!("Unknown operation"),
                };
                let v2 = match ops[2] {
                    "old" => Value::Old(),
                    _ => Value::Value(ops[2].parse().unwrap()),
                };
                operation = Some(MonkeyOperation { v1, op, v2 });
            }
            if line.starts_with("  Test: divisible by ") {
                test.divisible_by = line
                    .strip_prefix("  Test: divisible by ")
                    .unwrap()
                    .parse()
                    .unwrap();
            }
            if line.starts_with("    If true: throw to monkey ") {
                test.if_true_throw_to_monkey = line
                    .strip_prefix("    If true: throw to monkey ")
                    .unwrap()
                    .parse()
                    .unwrap();
            }
            if line.starts_with("    If false: throw to monkey ") {
                test.if_false_throw_to_monkey = line
                    .strip_prefix("    If false: throw to monkey ")
                    .unwrap()
                    .parse()
                    .unwrap();
            }
        }
        Ok(Self {
            items,
            operation: operation.expect("Monkey must have an operation defined."),
            test,
            items_inspections: 0,
        })
    }
}

struct GameKeepAway {
    monkeys: Vec<Monkey>,
}

impl GameKeepAway {
    fn new() -> Self {
        Self { monkeys: vec![] }
    }

    pub fn add_monkey(&mut self, monkey: Monkey) {
        self.monkeys.push(monkey);
    }

    pub fn round(&mut self) {
        for i in 0..self.monkeys.len() {
            let monkey = self.monkeys.get_mut(i).unwrap();

            let mut receivers: Vec<(usize, Item)> = Vec::new();
            while monkey.has_items() {
                let (to_monkey, item) = monkey.throw_item();
                receivers.push((to_monkey, item));
            }

            for (to_monkey, item) in receivers {
                let monkey_receiver = self.monkeys.get_mut(to_monkey).unwrap();
                monkey_receiver.receive_item(item);
            }
        }
    }
}

fn add_monkeys_from_input_file(game: &mut GameKeepAway) {
    let file = FSFile::open("./input.prod").expect("input file should exist");
    let mut monkey_lines = Vec::new();
    for line_wrapped in BufReader::new(file).lines() {
        let line = line_wrapped.unwrap();
        if line.is_empty() {
            game.add_monkey(Monkey::from_str(&monkey_lines.join("\n")).unwrap());
            monkey_lines.clear();
        } else {
            monkey_lines.push(line);
        }
    }
    game.add_monkey(Monkey::from_str(&monkey_lines.join("\n")).unwrap())
}

fn main() {
    let mut game = GameKeepAway::new();
    add_monkeys_from_input_file(&mut game);

    for _ in 0..10_000 {
        game.round();
    }
    for (i, monkey) in game.monkeys.iter().enumerate() {
        println!("{i}: {:?}", monkey.items_inspections);
    }
}
