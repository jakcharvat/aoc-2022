use std::collections::HashSet;

use advent::{coord3::Coord3, parsers::parse_signed};
use nom::{
    character::complete::{char, line_ending},
    combinator::map,
    multi::separated_list0,
    sequence::{preceded, tuple},
    IResult,
};

pub fn parse_drop(input: &str) -> IResult<&str, HashSet<Coord3>> {
    map(separated_list0(line_ending, parse_coord), |coords| {
        coords.into_iter().collect::<HashSet<_>>()
    })(input)
}

fn parse_coord(input: &str) -> IResult<&str, Coord3> {
    map(
        tuple((
            parse_signed,
            preceded(char(','), parse_signed),
            preceded(char(','), parse_signed),
        )),
        |(x, y, z)| Coord3::new(x, y, z),
    )(input)
}
