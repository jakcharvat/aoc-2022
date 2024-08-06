use advent::parsers::parse_signed;
use dbg_pls::DebugPls;
use nom::{
    branch::alt,
    bytes::complete::{tag, take_while_m_n},
    combinator::map,
    multi::separated_list1,
    sequence::{preceded, tuple},
    IResult,
};

#[derive(Clone, Debug, DebugPls)]
pub struct Valve {
    pub label: String,
    pub flow_rate: isize,
    pub neighbours: Vec<String>,
}

pub fn parse_valve(input: &str) -> IResult<&str, Valve> {
    let valve_tag = || {
        map(take_while_m_n(2, 2, char::is_alphabetic), |s: &str| {
            s.to_string()
        })
    };

    map(
        tuple((
            preceded(tag("Valve "), valve_tag()),
            preceded(tag(" has flow rate="), parse_signed),
            preceded(
                alt((
                    tag("; tunnel leads to valve "),
                    tag("; tunnels lead to valves "),
                )),
                separated_list1(tag(", "), valve_tag()),
            ),
        )),
        |(label, flow_rate, neighbours)| Valve {
            label,
            flow_rate,
            neighbours,
        },
    )(input)
}
