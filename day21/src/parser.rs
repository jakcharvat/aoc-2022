use dbg_pls::DebugPls;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while_m_n},
    character::complete::{char, digit1, line_ending, space0},
    combinator::{map, map_res, value},
    multi::separated_list1,
    sequence::{delimited, separated_pair, tuple},
    IResult,
};

#[derive(Clone, Copy, DebugPls)]
pub enum Operation {
    Plus,
    Minus,
    Times,
    Div,
}

#[derive(Clone, DebugPls)]
pub enum Job {
    Number(i64),
    Operation(String, Operation, String),
}

#[derive(Clone, DebugPls)]
pub struct Monkey {
    pub name: String,
    pub job: Job,
}

pub fn parse_monkeys(input: &str) -> IResult<&str, Vec<Monkey>> {
    separated_list1(line_ending, parse_monkey)(input)
}

fn parse_monkey(input: &str) -> IResult<&str, Monkey> {
    let stripped = |parser| delimited(space0, parser, space0);

    let monkey_name = || {
        map(
            take_while_m_n(4, 4, |c| char::is_ascii_alphabetic(&c)),
            String::from,
        )
    };

    let num = || map_res(digit1, |s: &str| s.parse::<i64>());
    let op = alt((
        value(Operation::Plus, char('+')),
        value(Operation::Minus, char('-')),
        value(Operation::Times, char('*')),
        value(Operation::Div, char('/')),
    ));

    let job = alt((
        map(num(), Job::Number),
        map(
            tuple((monkey_name(), stripped(op), monkey_name())),
            |(monkey1, op, monkey2)| Job::Operation(monkey1, op, monkey2),
        ),
    ));

    map(
        separated_pair(monkey_name(), tag(": "), job),
        |(name, job)| Monkey { name, job },
    )(input)
}
