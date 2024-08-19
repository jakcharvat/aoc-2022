use nom::{branch::alt, character::complete::char, combinator::value, multi::fold_many1, IResult};

#[cfg(not(test))]
const INPUT: &str = include_str!("../in.txt");
#[cfg(test)]
const INPUT: &str = include_str!("../small-in.txt");

fn snafu_digit(input: &str) -> IResult<&str, i64> {
    alt((
        value(2, char('2')),
        value(1, char('1')),
        value(0, char('0')),
        value(-1, char('-')),
        value(-2, char('=')),
    ))(input)
}

fn snafu_to_num(snafu: &str) -> i64 {
    fold_many1(snafu_digit, || 0, |acc, curr| acc * 5 + curr)(snafu)
        .unwrap()
        .1
}

fn num_to_snafu(mut num: i64) -> String {
    let mut digits = Vec::new();

    while num.abs() > 0 {
        num += 2;

        digits.push(match num.rem_euclid(5) - 2 {
            -2 => '=',
            -1 => '-',
            0 => '0',
            1 => '1',
            2 => '2',
            d => unreachable!("Invalid digit ((mod 5) - 2): {} ({:?})", d, d),
        });

        num = num / 5;
    }

    digits.into_iter().rev().collect()
}

fn part1() -> String {
    let sum = INPUT.lines().map(snafu_to_num).sum::<i64>();
    num_to_snafu(sum)
}

fn main() {
    println!(r#"Part 1: "{}""#, part1())
}

#[cfg(test)]
mod tests {
    macro_rules! snafu_tests {
        ( $( ($decimal:expr, $snafu:expr) ),* ) => {
            $(
                paste::paste!(
                    #[test]
                    fn [<snafu_to_num_ $decimal>]() {
                        assert_eq!(super::snafu_to_num($snafu), $decimal);
                    }
                );

                paste::paste!(
                    #[test]
                    fn [<num_to_snafu_ $decimal>]() {
                        assert_eq!(super::num_to_snafu($decimal), $snafu);
                    }
                );
            )*
        }
    }

    snafu_tests![
        (1, "1"),
        (2, "2"),
        (3, "1="),
        (4, "1-"),
        (5, "10"),
        (6, "11"),
        (7, "12"),
        (8, "2="),
        (9, "2-"),
        (10, "20"),
        (15, "1=0"),
        (20, "1-0"),
        (2022, "1=11-2"),
        (12345, "1-0---0"),
        (314159265, "1121-1110-1=0")
    ];

    #[test]
    fn part1() {
        assert_eq!(super::part1(), "2=-1=0")
    }
}
