use std::{
    fmt::Display,
    io::{stdin, Read},
};

use parser::{parse_monkeys, Monkey};

mod parser;

struct PrintMonkeys<'a>(&'a Vec<Monkey>);
impl<'a> Display for PrintMonkeys<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for (idx, monkey) in self.0.iter().enumerate() {
            let items: Vec<_> = monkey.items.iter().map(|i| i.to_string()).collect();
            writeln!(f, "Monkey {}: {}", idx, items.join(", "))?;
        }

        Ok(())
    }
}

fn do_monkey_business<F>(
    monkeys: &Vec<Monkey>,
    iterations: usize,
    log: bool,
    mut adjust_worry: F,
) -> usize
where
    F: FnMut(usize) -> usize,
{
    let mut items: Vec<_> = monkeys.iter().map(|m| m.items.clone()).collect();
    let mut inspections: Vec<_> = monkeys.iter().map(|_| 0usize).collect();

    for i in 0..iterations {
        for (src_monkey_idx, monkey) in monkeys.iter().enumerate() {
            while let Some(item) = items[src_monkey_idx].pop_front() {
                inspections[src_monkey_idx] += 1;
                let item = adjust_worry(monkey.op.as_ref()(item));
                let tgt_monkey_idx = match item % monkey.div_condition == 0 {
                    true => monkey.true_monkey,
                    false => monkey.false_monkey,
                };
                items[tgt_monkey_idx].push_back(item)
            }
        }

        if log {
            eprintln!("After round {}:\n{}", i + 1, PrintMonkeys(&monkeys))
        }
    }

    inspections.sort();
    inspections.reverse();

    inspections[0] * inspections[1]
}

fn main() {
    let mut source = String::new();
    stdin().read_to_string(&mut source).unwrap();

    let (_, monkeys) = parse_monkeys(&source).unwrap();
    println!(
        "Part 1: {}",
        do_monkey_business(&monkeys, 20, true, |x| x / 3)
    );

    let modulus = monkeys.iter().map(|m| m.div_condition).product::<usize>();
    println!(
        "Part 2: {}",
        do_monkey_business(&monkeys, 10000, false, move |x| x % modulus)
    );
}
