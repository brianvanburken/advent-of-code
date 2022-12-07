use std::{collections::HashMap, fs};

fn main() {
    let input = fs::read_to_string("./input.txt").expect("Expected input file to be there.");

    let mut commands: HashMap<String, Command> = HashMap::new();
    let mut wire: HashMap<String, u16> = HashMap::new();
    for line in input.lines() {
        let (lhs, variable) = line
            .split_once(" -> ")
            .expect("Expected a instruction with arrow");
        let command = Command::from(lhs);
        commands.insert(variable.to_owned(), command);
    }
    let result_part_1 = part_1(&mut wire, &commands, &"a".into());
    println!("Part 1: {:?}", result_part_1);
}

fn part_1(wire: &mut HashMap<String, u16>, commands: &HashMap<String, Command>, var: &Term) -> u16 {
    let var = match var {
        Term::Num(n) => return *n,
        Term::Var(s) => s,
    };

    if let Some(value) = wire.get(var) {
        return *value;
    }

    let command = commands.get(var).unwrap();
    let value = match command {
        Command::Num(num) => *num,
        Command::Var(name) => part_1(wire, commands, &name.into()),
        Command::And(left, right) => {
            let left = part_1(wire, commands, left);
            let right = part_1(wire, commands, right);
            left & right
        }
        Command::Or(left, right) => {
            let left = part_1(wire, commands, left);
            let right = part_1(wire, commands, right);
            left | right
        }
        Command::Lshift(left, amt) => {
            let left = part_1(wire, commands, &left.into());
            left << *amt
        }
        Command::Rshift(right, amt) => {
            let right = part_1(wire, commands, &right.into());
            right >> *amt
        }
        Command::Not(name) => {
            let value = part_1(wire, commands, &name.into());
            !value
        }
    };

    wire.insert(var.clone(), value);

    value
}

#[derive(Debug)]
enum Command {
    Num(u16),
    Var(String),
    And(Term, Term),
    Or(Term, Term),
    Lshift(String, u16),
    Rshift(String, u16),
    Not(String),
}

impl From<&str> for Command {
    fn from(s: &str) -> Self {
        let parts = s.split_whitespace().into_iter().collect::<Vec<&str>>();
        match parts[..] {
            [var1, "OR", var2] => Self::Or(var1.into(), var2.into()),
            [var1, "AND", var2] => Self::And(var1.into(), var2.into()),
            [var, "RSHIFT", num] => Self::Rshift(
                var.to_owned(),
                num.parse::<u16>().expect("Expected a num to rhs of RSHIFT"),
            ),
            [var, "LSHIFT", num] => Self::Lshift(
                var.to_owned(),
                num.parse::<u16>().expect("Expected a num to rhs of LSHIFT"),
            ),
            ["NOT", var] => Self::Not(var.to_owned()),
            [num] if is_numeric(num) => {
                Self::Num(num.parse::<u16>().expect("Expected to parse number"))
            }
            [var] => Self::Var(var.to_owned()),
            _ => panic!("Expected a valid action"),
        }
    }
}

#[derive(Debug)]
enum Term {
    Var(String),
    Num(u16),
}

impl From<&str> for Term {
    fn from(s: &str) -> Self {
        match s.parse::<u16>() {
            Ok(num) => Self::Num(num),
            Err(_) => Self::Var(s.to_owned()),
        }
    }
}

impl From<&String> for Term {
    fn from(s: &String) -> Self {
        match s.parse::<u16>() {
            Ok(num) => Self::Num(num),
            Err(_) => Self::Var(s.to_owned()),
        }
    }
}

fn is_numeric(s: &str) -> bool {
    s.chars().all(|c| c.is_numeric())
}
