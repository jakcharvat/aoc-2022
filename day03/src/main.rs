use std::collections::HashSet;

#[cfg(not(test))]
const INPUT: &str = include_str!("../in.txt");
#[cfg(test)]
const INPUT: &str = include_str!("../small-in.txt");

fn items(rucksack: &str) -> HashSet<u8> {
    rucksack.bytes().collect()
}

fn priority(ch: u8) -> i64 {
    (if ch.is_ascii_lowercase() {
        ch - b'a' + 1
    } else {
        ch - b'A' + 27
    }) as i64
}

fn part1() -> i64 {
    INPUT
        .lines()
        .map(|r| {
            let (a, b) = r.split_at(r.len() / 2);
            let a = items(a);
            let b = items(b);

            let mut intersect = a.intersection(&b);
            assert_eq!(intersect.clone().count(), 1);
            *intersect.next().unwrap()
        })
        .map(priority)
        .sum()
}

fn part2() -> i64 {
    INPUT
        .lines()
        .collect::<Vec<_>>()
        .chunks(3)
        .map(|g| {
            let a = items(g[0]);
            let b = items(g[1]);
            let ab = a.intersection(&b).copied().collect::<HashSet<_>>();

            let c = items(g[2]);
            let mut abc = ab.intersection(&c).copied();

            assert_eq!(abc.clone().count(), 1);
            abc.next().unwrap()
        })
        .map(priority)
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
        assert_eq!(super::part1(), 157);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(), 70);
    }
}
