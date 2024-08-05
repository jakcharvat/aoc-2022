use advent::{coord::Coord, parsers::parse_signed};
use nom::{
    bytes::complete::tag,
    combinator::map,
    sequence::{pair, preceded, separated_pair},
    IResult,
};

#[derive(Clone, Copy, Debug, dbg_pls::DebugPls)]
pub struct SensorData {
    pub sensor_pos: Coord,
    pub beacon_pos: Coord,
}

pub fn parse_sensor(input: &str) -> IResult<&str, SensorData> {
    map(
        pair(
            preceded(tag("Sensor at "), parse_coord),
            preceded(tag(": closest beacon is at "), parse_coord),
        ),
        |(sensor_pos, beacon_pos)| SensorData {
            sensor_pos,
            beacon_pos,
        },
    )(input)
}

fn parse_coord(input: &str) -> IResult<&str, Coord> {
    map(
        separated_pair(
            preceded(tag("x="), parse_signed),
            tag(", "),
            preceded(tag("y="), parse_signed),
        ),
        |(x, y)| Coord::new(x, y),
    )(input)
}
