use std::cell::RefCell;
use std::collections::HashMap;
use std::fs::File as FSFile;
use std::io::{BufRead, BufReader};
use std::rc::Rc;
use std::str::FromStr;

type Name = String;

#[derive(Debug)]
enum Expression {
    Val(i64),
    Str(String),
}

#[derive(Debug, Hash, Eq, Clone, PartialEq)]
enum Operation {
    Value(i64),
    Add(Name, Name),
    Mul(Name, Name),
    Sub(Name, Name),
    Div(Name, Name),
}

impl Operation {
    fn variables(&self) -> Option<(Name, Name)> {
        match self {
            Operation::Value(_) => None,
            Operation::Add(a, b) => Some((a.to_string(), b.to_string())),
            Operation::Mul(a, b) => Some((a.to_string(), b.to_string())),
            Operation::Sub(a, b) => Some((a.to_string(), b.to_string())),
            Operation::Div(a, b) => Some((a.to_string(), b.to_string())),
        }
    }

    fn exec(&self, context: &HashMap<Name, i64>) -> i64 {
        match self {
            Operation::Value(v) => v.clone(),
            Operation::Add(a, b) => context.get(a).unwrap() + context.get(b).unwrap(),
            Operation::Mul(a, b) => context.get(a).unwrap() * context.get(b).unwrap(),
            Operation::Sub(a, b) => context.get(a).unwrap() - context.get(b).unwrap(),
            Operation::Div(a, b) => context.get(a).unwrap() / context.get(b).unwrap(),
        }
    }
}

impl FromStr for Operation {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if let Ok(value) = s.parse::<i64>() {
            return Ok(Operation::Value(value));
        } else if s.contains(" + ") {
            let (a, b) = s.split_once(" + ").unwrap();
            return Ok(Operation::Add(a.to_string(), b.to_string()));
        } else if s.contains(" - ") {
            let (a, b) = s.split_once(" - ").unwrap();
            return Ok(Operation::Sub(a.to_string(), b.to_string()));
        } else if s.contains(" * ") {
            let (a, b) = s.split_once(" * ").unwrap();
            return Ok(Operation::Mul(a.to_string(), b.to_string()));
        } else if s.contains(" / ") {
            let (a, b) = s.split_once(" / ").unwrap();
            return Ok(Operation::Div(a.to_string(), b.to_string()));
        } else {
            unreachable!("invalid data")
        }
    }
}

#[derive(Debug, Hash, Eq, PartialEq)]
struct Monkey {
    name: String,
    operation: Operation,
}

impl FromStr for Monkey {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, operation_raw) = s.split_once(": ").unwrap();
        let operation = Operation::from_str(operation_raw).unwrap();
        Ok(Monkey {
            name: name.to_string(),
            operation,
        })
    }
}

#[derive(PartialEq)]
struct TreeNode {
    pub value: Option<i64>,

    name: String,
    pub parent: Option<Rc<RefCell<TreeNode>>>,

    pub operation: Option<Operation>,
    pub left: Option<Rc<RefCell<TreeNode>>>,
    pub right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    pub fn new(name: String) -> TreeNode {
        return TreeNode {
            name,
            value: None,
            parent: None,
            operation: None,
            left: None,
            right: None,
        };
    }

    pub fn value_without(&self, name: String) -> Expression {
        if self.name == name {
            return Expression::Str(name);
        }
        if let Operation::Value(v) = self.operation.as_ref().unwrap() {
            return Expression::Val(v.clone());
        }

        let a = self
            .left
            .as_ref()
            .unwrap()
            .borrow()
            .value_without(name.clone());
        let b = self.right.as_ref().unwrap().borrow().value_without(name);

        if let Expression::Val(a) = a {
            if let Expression::Val(b) = b {
                let v = match self.operation.as_ref().unwrap() {
                    Operation::Value(v) => v.clone(),
                    Operation::Add(_, _) => a + b,
                    Operation::Sub(_, _) => a - b,
                    Operation::Mul(_, _) => a * b,
                    Operation::Div(_, _) => a / b,
                };
                return Expression::Val(v);
            }
        }

        let a = match a {
            Expression::Val(v) => v.to_string(),
            Expression::Str(s) => s,
        };
        let b = match b {
            Expression::Val(v) => v.to_string(),
            Expression::Str(s) => s,
        };

        let op = match self.operation.as_ref().unwrap() {
            Operation::Value(_) => unreachable!("invalid data"),
            Operation::Add(_, _) => "+",
            Operation::Sub(_, _) => "-",
            Operation::Mul(_, _) => "*",
            Operation::Div(_, _) => "/",
        };

        return Expression::Str(format!("({}{}{})", a, op, b));
    }

    pub fn value_equals(&self, x_name: Name, equal: i64) -> i64 {
        if self.name == x_name {
            return equal;
        }

        let mut left: Option<i64> = None;
        let mut right: Option<i64> = None;

        if let Expression::Val(v) = self
            .left
            .as_ref()
            .unwrap()
            .borrow()
            .value_without(x_name.clone())
        {
            left = Some(v);
        }
        if let Expression::Val(v) = self
            .right
            .as_ref()
            .unwrap()
            .borrow()
            .value_without(x_name.clone())
        {
            right = Some(v);
        }

        if left.is_none() && right.is_none() {
            return 0;
        }

        if left.is_some() && right.is_none() {
            // equals = left OP right
            let new_equals = match self.operation.as_ref().unwrap() {
                Operation::Value(_) => unreachable!("invalid data"),
                // right = equals - left
                Operation::Add(_, _) => equal - left.unwrap(),
                // right = left - equals
                Operation::Sub(_, _) => left.unwrap() - equal,
                // right = equals / left
                Operation::Mul(_, _) => equal / left.unwrap(),
                // right = left / equals
                Operation::Div(_, _) => left.unwrap() / equal,
            };
            return self
                .right
                .as_ref()
                .unwrap()
                .borrow()
                .value_equals(x_name.clone(), new_equals);
        }

        if right.is_some() && left.is_none() {
            // equals = left OP right
            let new_equals = match self.operation.as_ref().unwrap() {
                Operation::Value(_) => unreachable!("invalid data"),
                // left = equals - right
                Operation::Add(_, _) => equal - right.unwrap(),
                // left = right + equals
                Operation::Sub(_, _) => equal + right.unwrap(),
                // left = equals / right
                Operation::Mul(_, _) => equal / right.unwrap(),
                // left = equals * right
                Operation::Div(_, _) => equal * right.unwrap(),
            };
            return self
                .left
                .as_ref()
                .unwrap()
                .borrow()
                .value_equals(x_name.clone(), new_equals);
        }

        0
    }

    pub fn find_value(&self, name: String) -> i64 {
        let a = self
            .left
            .as_ref()
            .unwrap()
            .borrow()
            .value_without(name.clone());
        if let Expression::Val(a) = a {
            return self.right.as_ref().unwrap().borrow().value_equals(name, a);
        }

        let b = self
            .right
            .as_ref()
            .unwrap()
            .borrow()
            .value_without(name.clone());
        if let Expression::Val(b) = b {
            return self.left.as_ref().unwrap().borrow().value_equals(name, b);
        }

        0
    }

    pub fn value(&self) -> i64 {
        if let Some(v) = self.value {
            return v;
        } else {
            if let Operation::Value(v) = self.operation.as_ref().unwrap() {
                return *v;
            }

            let a = self.left.as_ref().unwrap().borrow().value();
            let b = self.right.as_ref().unwrap().borrow().value();

            return match self.operation.as_ref().unwrap() {
                Operation::Value(v) => *v,
                Operation::Add(_, _) => a + b,
                Operation::Sub(_, _) => a - b,
                Operation::Mul(_, _) => a * b,
                Operation::Div(_, _) => a / b,
            };
        }
    }
}

struct MonkeyTreeBuilder {
    monkeys: HashMap<Name, Monkey>,
}

impl MonkeyTreeBuilder {
    fn new() -> Self {
        MonkeyTreeBuilder {
            monkeys: HashMap::new(),
        }
    }

    fn add_monkey(&mut self, monkey: Monkey) {
        self.monkeys.insert(monkey.name.clone(), monkey);
    }

    pub fn build(&self, name: Name) -> Rc<RefCell<TreeNode>> {
        let root = Rc::new(RefCell::new(TreeNode::new(name.clone())));
        let monkey = self.monkeys.get(&name).unwrap();

        root.borrow_mut().operation = Some(monkey.operation.clone());
        if !monkey.operation.variables().is_none() {
            let (left, right) = monkey.operation.variables().unwrap();
            root.borrow_mut().left = Some(self.build(left));
            root.borrow_mut().right = Some(self.build(right));
        }

        root
    }
}

fn main() {
    let mut tree_builder = MonkeyTreeBuilder::new();

    let file = FSFile::open("./input.prod").expect("input file should exist");
    for line_wrapped in BufReader::new(file).lines() {
        let monkey = Monkey::from_str(&line_wrapped.unwrap()).unwrap();
        tree_builder.add_monkey(monkey);
    }

    // println!("root monkey: {}", monkeys.yell())
    let tree = tree_builder.build("root".to_string());
    println!("{:?}", tree.borrow().value());
    println!("{:?}", tree.borrow().find_value("humn".to_string()));
}
