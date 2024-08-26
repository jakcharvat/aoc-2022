use nom::{
    character::complete::{char, digit1},
    sequence::separated_pair,
    IResult,
};

#[cfg(not(test))]
const INPUT: &str = include_str!("../in.txt");
#[cfg(test)]
const INPUT: &str = include_str!("../small-in.txt");

type SectionRange = (i64, i64);

fn parse_range(input: &str) -> IResult<&str, SectionRange> {
    let (input, (start, end)) = separated_pair(digit1, char('-'), digit1)(input)?;
    Ok((input, (start.parse().unwrap(), end.parse().unwrap())))
}

fn parse_line(input: &str) -> IResult<&str, (SectionRange, SectionRange)> {
    separated_pair(parse_range, char(','), parse_range)(input)
}

fn fully_contains(inner: SectionRange, outer: SectionRange) -> bool {
    inner.0 >= outer.0 && inner.1 <= outer.1
}

fn overlaps(a: SectionRange, b: SectionRange) -> bool {
    let inside = |point: i64, interval: SectionRange| fully_contains((point, point), interval);
    fully_contains(a, b) || fully_contains(b, a) || inside(a.0, b) || inside(a.1, b)
}

fn part1() -> usize {
    INPUT
        .lines()
        .map(|l| parse_line(&l).unwrap().1)
        .filter(|&(a, b)| fully_contains(a, b) || fully_contains(b, a))
        .count()
}

fn part2() -> usize {
    INPUT
        .lines()
        .map(|l| parse_line(&l).unwrap().1)
        .filter(|&(a, b)| overlaps(a, b))
        .count()
}

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        assert_eq!(super::part1(), 2);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(), 4);
    }
}
