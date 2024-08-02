use std::collections::VecDeque;

use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{char, digit1, line_ending, space0},
    combinator::{map, map_res, opt, value, verify},
    multi::separated_list1,
    sequence::{delimited, pair, tuple},
    IResult,
};

pub struct Monkey {
    pub items: VecDeque<usize>,
    pub op: Box<dyn Fn(usize) -> usize>,
    pub div_condition: usize,
    pub true_monkey: usize,
    pub false_monkey: usize,
}

pub fn dbg_dmp<'a, F, O, E: std::fmt::Debug>(
    f: F,
    context: &'static str,
) -> impl Fn(&'a str) -> IResult<&'a str, O, E>
where
    F: Fn(&'a str) -> IResult<&'a str, O, E>,
{
    move |i: &'a str| match f(i) {
        Err(e) => {
            println!("{}: Error({:?}) at:\n{}", context, e, i);
            Err(e)
        }
        a => a,
    }
}

pub fn parse_monkeys(mut input: &str) -> IResult<&str, Vec<Monkey>> {
    let mut monkeys: Vec<Monkey> = Vec::new();
    loop {
        let (new_input, monkey) = opt(dbg_dmp(parse_monkey(monkeys.len()), "Opt monkey"))(input)?;
        eprintln!("Parsed monkey: {}", monkey.is_some());
        let Some(monkey) = monkey else {
            return Ok((new_input, monkeys));
        };
        let (new_input, _) = opt(line_ending)(new_input)?;

        monkeys.push(monkey);
        input = new_input;
    }
}

fn parse_num(input: &str) -> IResult<&str, usize> {
    map_res(digit1, |s: &str| s.parse::<usize>())(input)
}

fn parse_monkey(monkey_idx: usize) -> impl Fn(&str) -> IResult<&str, Monkey> {
    move |input| -> IResult<&str, Monkey> {
        let strip_nl = || tuple((space0, line_ending, space0));
        let comma_sep = || tuple((space0, char(','), space0));

        let (input, _) = delimited(
            tag("Monkey "),
            verify(parse_num, |idx| *idx == monkey_idx),
            pair(tag(":"), strip_nl()),
        )(input)?;

        let (input, items) = delimited(
            tag("Starting items: "),
            separated_list1(comma_sep(), parse_num),
            strip_nl(),
        )(input)?;

        let (input, op) = delimited(tag("Operation: new = "), parse_expr, strip_nl())(input)?;
        let (input, div_condition) =
            delimited(tag("Test: divisible by "), parse_num, strip_nl())(input)?;
        let (input, true_monkey) =
            delimited(tag("If true: throw to monkey "), parse_num, strip_nl())(input)?;
        let (input, false_monkey) =
            delimited(tag("If false: throw to monkey "), parse_num, strip_nl())(input)?;

        Ok((
            input,
            Monkey {
                items: VecDeque::from(items),
                op,
                div_condition,
                true_monkey,
                false_monkey,
            },
        ))
    }
}

#[derive(Clone, Copy)]
enum Term {
    OldRef,
    Constant(usize),
}

#[derive(Clone, Copy)]
enum Op {
    Times,
    Plus,
}

fn parse_expr(input: &str) -> IResult<&str, Box<dyn Fn(usize) -> usize>> {
    let term = || {
        alt((
            value(Term::OldRef, tag("old")),
            map(parse_num, Term::Constant),
        ))
    };

    let (input, lhs) = term()(input)?;
    let (input, op) = delimited(
        space0,
        opt(alt((
            value(Op::Times, char('*')),
            value(Op::Plus, char('+')),
        ))),
        space0,
    )(input)?;

    let Some(op) = op else {
        return Ok((
            input,
            match lhs {
                Term::OldRef => Box::new(|x| x),
                Term::Constant(val) => Box::new(move |_| val),
            },
        ));
    };

    let (input, rhs) = term()(input)?;

    let f = match op {
        Op::Times => |x: usize, y: usize| x * y,
        Op::Plus => |x: usize, y: usize| x + y,
    };

    Ok((
        input,
        match (lhs, rhs) {
            (Term::OldRef, Term::OldRef) => Box::new(move |x| f(x, x)),
            (Term::OldRef, Term::Constant(val)) | (Term::Constant(val), Term::OldRef) => {
                Box::new(move |x| f(val, x))
            }
            (Term::Constant(a), Term::Constant(b)) => Box::new(move |_| f(a, b)),
        },
    ))
}
