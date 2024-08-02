use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, space1},
    combinator::{all_consuming, map, map_res, opt, value},
    sequence::{pair, preceded},
    IResult,
};

#[derive(Debug, Clone)]
pub enum Command {
    Noop,
    Addx(i64),
}

pub fn parse_command(input: &str) -> IResult<&str, Command> {
    all_consuming(alt((parse_noop, parse_addx)))(input)
}

fn parse_noop(input: &str) -> IResult<&str, Command> {
    value(Command::Noop, tag("noop"))(input)
}

fn parse_addx(input: &str) -> IResult<&str, Command> {
    let instr = pair(tag("addx"), space1);
    map(preceded(instr, parse_num), |num| Command::Addx(num))(input)
}

fn parse_num(input: &str) -> IResult<&str, i64> {
    let negative = value(-1i64, tag("-"));
    let positive = value(1i64, opt(tag("+")));
    let (input, sign) = alt((negative, positive))(input)?;

    let (input, number) = map_res(digit1, |num: &str| num.parse::<i64>())(input)?;
    Ok((input, sign * number))
}
