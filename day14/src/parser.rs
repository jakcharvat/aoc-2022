use advent::coord::Coord;
use nom::{
    bytes::complete::tag,
    character::complete::{char, digit1, line_ending},
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::separated_pair,
    IResult,
};

pub fn parse_lines(input: &str) -> IResult<&str, Vec<Vec<Coord>>> {
    separated_list1(line_ending, parse_line)(input)
}

pub fn parse_line(input: &str) -> IResult<&str, Vec<Coord>> {
    separated_list1(tag(" -> "), parse_coord)(input)
}

fn parse_coord(input: &str) -> IResult<&str, Coord> {
    map(separated_pair(parse_num, char(','), parse_num), |(x, y)| {
        Coord::new(x, y)
    })(input)
}

fn parse_num(input: &str) -> IResult<&str, isize> {
    map_res(digit1, |s: &str| s.parse::<isize>())(input)
}
