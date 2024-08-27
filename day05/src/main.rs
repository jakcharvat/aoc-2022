use std::collections::VecDeque;

#[allow(unused_imports)]
use dbg_pls::pretty;
use nom::{
    bytes::complete::tag,
    character::complete::digit1,
    combinator::{map, map_res},
    sequence::{preceded, tuple},
    IResult,
};

#[cfg(not(test))]
const INPUT: &str = include_str!("../in.txt");
#[cfg(test)]
const INPUT: &str = include_str!("../small-in.txt");

type Stacks = Vec<VecDeque<char>>;

fn stacks() -> (Stacks, Vec<&'static str>) {
    let lines = INPUT.lines().collect::<Vec<_>>();
    let split_idx = lines.iter().position(|l| l.is_empty()).unwrap();

    let instructions = Vec::from(&lines[(split_idx + 1)..]);
    let stack_lines = &lines[..(split_idx - 1)];
    let stacks_count = (stack_lines.last().unwrap().len() + 1) / 4;
    let mut stacks = vec![VecDeque::<char>::new(); stacks_count];

    for line in stack_lines.into_iter().rev() {
        let chars = line.chars().collect::<Vec<_>>();
        for (stack_i, ch) in chars.into_iter().skip(1).step_by(4).enumerate() {
            if ch != ' ' {
                stacks[stack_i].push_back(ch)
            };
        }
    }

    (stacks, instructions)
}

struct Instruction {
    count: usize,
    from: usize,
    to: usize,
}

fn parse_instruction(input: &str) -> IResult<&str, Instruction> {
    let num = || map_res(digit1, |s: &str| s.parse());

    map(
        tuple((
            preceded(tag("move "), num()),
            preceded(tag(" from "), num()),
            preceded(tag(" to "), num()),
        )),
        |(count, from, to)| Instruction {
            count,
            from: from - 1,
            to: to - 1,
        },
    )(input)
}

fn run(mut apply: impl FnMut(&mut Stacks, Instruction)) -> String {
    let (mut stacks, instructions) = stacks();

    for ins in instructions {
        let ins = parse_instruction(ins).unwrap().1;
        apply(&mut stacks, ins);
    }

    stacks
        .into_iter()
        .map(|s| *s.back().unwrap())
        .collect::<String>()
}

fn part1() -> String {
    run(|stacks, ins| {
        for _ in 0..ins.count {
            let c = stacks[ins.from].pop_back().unwrap();
            stacks[ins.to].push_back(c);
        }
    })
}

fn part2() -> String {
    run(|stacks, ins| {
        let split_idx = stacks[ins.from].len() - ins.count;
        let mut taking = stacks[ins.from].split_off(split_idx);
        stacks[ins.to].append(&mut taking);
    })
}

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        assert_eq!(super::part1(), "CMZ");
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(), "MCD");
    }
}
