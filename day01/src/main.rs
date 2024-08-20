#[cfg(not(test))]
const INPUT: &str = include_str!("../in.txt");
#[cfg(test)]
const INPUT: &str = include_str!("../small-in.txt");

fn elves() -> Vec<i64> {
    let lines = INPUT.lines().collect::<Vec<_>>();
    let elves = lines
        .chunk_by(|line, _| !line.is_empty())
        .map(|elf| elf.iter().filter_map(|f| f.parse::<i64>().ok()).sum())
        .collect::<Vec<_>>();

    elves
}

fn part1() -> i64 {
    elves().into_iter().max().unwrap()
}

fn part2() -> i64 {
    let mut elves = elves();
    elves.sort();
    elves.reverse();

    elves.into_iter().take(3).sum()
}

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        assert_eq!(super::part1(), 24000);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(), 45000);
    }
}
