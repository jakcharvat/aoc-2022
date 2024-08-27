use std::collections::BTreeSet;

fn find_marker(input: &str, len: usize) -> usize {
    let chars = input.chars().collect::<Vec<_>>();

    (0..)
        .position(|i| {
            i >= (len - 1) && chars[i + 1 - len..=i].iter().collect::<BTreeSet<_>>().len() == len
        })
        .unwrap()
        + 1
}

fn part1(input: &str) -> usize {
    find_marker(input, 4)
}

fn part2(input: &str) -> usize {
    find_marker(input, 14)
}

fn main() {
    let input = include_str!("../in.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        assert_eq!(super::part1("bvwbjplbgvbhsrlpgdmjqwftvncz"), 5);
        assert_eq!(super::part1("nppdvjthqldpwncqszvftbrmjlhg"), 6);
        assert_eq!(super::part1("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 10);
        assert_eq!(super::part1("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 11);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2("mjqjpqmgbljsphdztnvjfqwrcgsmlb"), 19);
        assert_eq!(super::part2("bvwbjplbgvbhsrlpgdmjqwftvncz"), 23);
        assert_eq!(super::part2("nppdvjthqldpwncqszvftbrmjlhg"), 23);
        assert_eq!(super::part2("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg"), 29);
        assert_eq!(super::part2("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw"), 26);
    }
}
