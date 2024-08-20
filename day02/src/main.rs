use nom::{
    character::complete::{char, one_of},
    combinator::map,
    sequence::separated_pair,
    IResult,
};

#[cfg(not(test))]
const INPUT: &str = include_str!("../in.txt");
#[cfg(test)]
const INPUT: &str = include_str!("../small-in.txt");

fn player(input: &str) -> IResult<&str, i64> {
    map(one_of("XYZ"), |c: char| (c as u8 - b'X') as i64)(input)
}

fn opponent(input: &str) -> IResult<&str, i64> {
    map(one_of("ABC"), |c: char| (c as u8 - b'A') as i64)(input)
}

fn parse_line(line: &str) -> (i64, i64) {
    separated_pair(opponent, char(' '), player)(line).unwrap().1
}

fn calc_score((opponent, player): (i64, i64)) -> i64 {
    player + 1 + (player - opponent + 1).rem_euclid(3) * 3
}

fn make_play((opponent, res): (i64, i64)) -> (i64, i64) {
    (opponent, (opponent - 1 + res).rem_euclid(3))
}

fn part1() -> i64 {
    INPUT.lines().map(parse_line).map(calc_score).sum()
}

fn part2() -> i64 {
    INPUT
        .lines()
        .map(parse_line)
        .map(make_play)
        .map(calc_score)
        .sum()
}

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        assert_eq!(super::part1(), 15);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(), 12);
    }
}
