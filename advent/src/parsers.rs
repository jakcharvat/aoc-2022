use nom::{
    branch::alt,
    character::{complete::char, complete::digit1},
    combinator::{map, map_res, opt, value},
    IResult,
};

pub fn parse_signed(input: &str) -> IResult<&str, isize> {
    let minus = value(-1isize, char('-'));
    let plus = value(1isize, opt(char('+')));
    let (input, mul) = alt((minus, plus))(input)?;

    map(map_res(digit1, |s: &str| s.parse::<isize>()), move |num| {
        num * mul
    })(input)
}
