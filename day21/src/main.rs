use std::collections::HashMap;

#[allow(unused_imports)]
use dbg_pls::pretty;

use parser::{parse_monkeys, Job, Operation};

mod parser;

const ROOT_MONKEY: &str = "root";
const HUMAN: &str = "humn";

fn eval_rec(
    monkey_name: &str,
    monkeys: &HashMap<String, Job>,
    memo: &mut HashMap<String, i64>,
    ignore_human: bool,
) -> Option<i64> {
    if ignore_human && monkey_name == HUMAN {
        return None;
    }

    if memo.contains_key(monkey_name) {
        return Some(*memo.get(monkey_name).unwrap());
    }

    let val = match monkeys.get(monkey_name).unwrap() {
        Job::Number(num) => *num,
        Job::Operation(monkey1, op, monkey2) => {
            let val1 = eval_rec(monkey1, monkeys, memo, ignore_human);
            let val2 = eval_rec(monkey2, monkeys, memo, ignore_human);
            let (val1, val2) = (val1?, val2?);

            match op {
                Operation::Plus => val1 + val2,
                Operation::Minus => val1 - val2,
                Operation::Times => val1 * val2,
                Operation::Div => {
                    assert_eq!(val1 % val2, 0);
                    val1 / val2
                }
            }
        }
    };

    memo.insert(monkey_name.to_string(), val);
    Some(val)
}

fn part1(input: &str) -> i64 {
    let monkeys = parse_monkeys(input).unwrap().1;
    let monkeys: HashMap<_, _> = monkeys
        .into_iter()
        .map(|monkey| (monkey.name, monkey.job))
        .collect();

    let mut memo = HashMap::new();
    eval_rec(ROOT_MONKEY, &monkeys, &mut memo, false).unwrap()
}

fn eval_human(
    monkey_name: &str,
    target_value: i64,
    monkeys: &HashMap<String, Job>,
    memo: &HashMap<String, i64>,
) -> i64 {
    assert!(
        !memo.contains_key(monkey_name),
        "Attempting to evaluate known monkey {}",
        monkey_name
    );

    eprintln!("Attempting to make {} = {}", monkey_name, target_value);

    if monkey_name == HUMAN {
        return target_value;
    }

    let Job::Operation(child1, op, child2) = monkeys.get(monkey_name).unwrap() else {
        panic!("Attempting to fit non-operation monkey {}", monkey_name)
    };

    let (child_name, child_target) = if let Some(&val) = memo.get(child1) {
        (
            child2,
            match op {
                Operation::Plus => target_value - val,
                Operation::Minus => val - target_value,
                Operation::Times => {
                    assert_eq!(target_value % val, 0);
                    target_value / val
                }
                Operation::Div => {
                    assert_eq!(val % target_value, 0);
                    val / target_value
                }
            },
        )
    } else {
        let val = *memo.get(child2).unwrap();
        (
            child1,
            match op {
                Operation::Plus => target_value - val,
                Operation::Minus => target_value + val,
                Operation::Times => {
                    assert_eq!(target_value % val, 0);
                    target_value / val
                }
                Operation::Div => target_value * val,
            },
        )
    };

    eval_human(child_name, child_target, monkeys, memo)
}

fn part2(input: &str) -> i64 {
    let monkeys = parse_monkeys(input).unwrap().1;
    let monkeys: HashMap<_, _> = monkeys
        .into_iter()
        .map(|monkey| (monkey.name, monkey.job))
        .collect();

    let mut memo = HashMap::new();
    eval_rec(ROOT_MONKEY, &monkeys, &mut memo, true);

    let Job::Operation(child1, _, child2) = monkeys.get(ROOT_MONKEY).unwrap() else {
        panic!("Root monkey is a leaf")
    };

    let (search_monkey, target_value) = match memo.get(child1) {
        Some(val) => (child2, *val),
        None => (child1, *memo.get(child2).unwrap()),
    };

    pretty!(&memo);
    eval_human(search_monkey, target_value, &monkeys, &memo)
}

fn main() {
    let input = include_str!("../in.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("../small-in.txt");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 152);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), 301);
    }
}
