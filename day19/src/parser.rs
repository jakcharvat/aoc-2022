use nom::{
    bytes::complete::{tag, take_while1},
    character::complete::{char, digit1},
    combinator::{map, map_res},
    multi::separated_list1,
    sequence::{delimited, preceded, separated_pair, tuple},
    IResult,
};

pub type Materials = [u32; 4];
pub type Blueprint = [Materials; 4];

#[macro_export]
macro_rules! get {
    ( ore $arr:expr ) => {
        $arr[0]
    };

    ( clay $arr:expr ) => {
        $arr[1]
    };

    ( obs $arr:expr ) => {
        $arr[2]
    };

    ( geo $arr:expr ) => {
        $arr[3]
    };
}

pub fn parse_blueprints(input: &str) -> IResult<&str, Vec<Blueprint>> {
    separated_list1(ws, parse_blueprint)(input)
}

pub fn parse_blueprint(input: &str) -> IResult<&str, Blueprint> {
    map(
        preceded(
            tuple((tag("Blueprint "), digit1, char(':'))),
            tuple((
                preceded(
                    ws,
                    delimited(tag("Each ore robot costs "), num, tag(" ore.")),
                ),
                preceded(
                    ws,
                    delimited(tag("Each clay robot costs "), num, tag(" ore.")),
                ),
                preceded(
                    ws,
                    delimited(
                        tag("Each obsidian robot costs "),
                        separated_pair(num, tag(" ore and "), num),
                        tag(" clay."),
                    ),
                ),
                preceded(
                    ws,
                    delimited(
                        tag("Each geode robot costs "),
                        separated_pair(num, tag(" ore and "), num),
                        tag(" obsidian."),
                    ),
                ),
            )),
        ),
        |(ore, clay, obsidian, geode)| {
            [
                [ore, 0, 0, 0],
                [clay, 0, 0, 0],
                [obsidian.0, obsidian.1, 0, 0],
                [geode.0, 0, geode.1, 0],
            ]
        },
    )(input)
}

pub fn ws(input: &str) -> IResult<&str, &str> {
    take_while1(char::is_whitespace)(input)
}

pub fn num(input: &str) -> IResult<&str, u32> {
    map_res(digit1, |s: &str| s.parse::<u32>())(input)
}
