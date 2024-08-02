use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{digit1, line_ending},
    combinator::{map, map_res},
    multi::{many_m_n, separated_list0, separated_list1},
    sequence::{delimited, separated_pair},
    IResult,
};

#[derive(Clone, PartialEq, Eq)]
pub enum Packet {
    List(Vec<Packet>),
    Atom(u8),
}

pub fn dbg_dmp<'a, F, O, E: std::fmt::Debug>(
    mut f: F,
    context: &'static str,
) -> impl FnMut(&'a str) -> IResult<&'a str, O, E>
where
    F: FnMut(&'a str) -> IResult<&'a str, O, E>,
{
    move |i: &'a str| match f(i) {
        Err(e) => {
            println!("{}: Error({:?}) at:\n{}", context, e, i);
            Err(e)
        }
        a => a,
    }
}

pub fn parse_packet_pairs(input: &str) -> IResult<&str, Vec<(Packet, Packet)>> {
    dbg_dmp(
        separated_list1(many_m_n(2, 2, line_ending), parse_packet_pair),
        "Parse Packet Pairs",
    )(input)
}

fn parse_packet_pair(input: &str) -> IResult<&str, (Packet, Packet)> {
    dbg_dmp(
        separated_pair(parse_packet, line_ending, parse_packet),
        "Parse Packet Pair",
    )(input)
}

pub fn parse_packet(input: &str) -> IResult<&str, Packet> {
    alt((
        map(parse_num, Packet::Atom),
        map(
            delimited(tag("["), separated_list0(tag(","), parse_packet), tag("]")),
            Packet::List,
        ),
    ))(input)
}

fn parse_num(input: &str) -> IResult<&str, u8> {
    map_res(digit1, |s: &str| s.parse::<u8>())(input)
}
