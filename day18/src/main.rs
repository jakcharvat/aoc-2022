use std::collections::{HashSet, VecDeque};

use advent::coord3::Coord3;
use parser::parse_drop;

#[allow(unused_imports)]
use dbg_pls::pretty;

mod parser;

const NEIGHBOURS: [Coord3; 6] = [
    Coord3 { x: 0, y: 0, z: 1 },
    Coord3 { x: 0, y: 0, z: -1 },
    Coord3 { x: 0, y: 1, z: 0 },
    Coord3 { x: 0, y: -1, z: 0 },
    Coord3 { x: 1, y: 0, z: 0 },
    Coord3 { x: -1, y: 0, z: 0 },
];

fn bounding_box(drop: &HashSet<Coord3>) -> (Coord3, Coord3) {
    let min = drop
        .iter()
        .copied()
        .reduce(|acc, curr| Coord3 {
            x: acc.x.min(curr.x),
            y: acc.y.min(curr.y),
            z: acc.z.min(curr.z),
        })
        .unwrap();

    let max = drop
        .iter()
        .copied()
        .reduce(|acc, curr| Coord3 {
            x: acc.x.max(curr.x),
            y: acc.y.max(curr.y),
            z: acc.z.max(curr.z),
        })
        .unwrap();

    (min, max)
}

fn part1(input: &str) -> isize {
    let drop = parse_drop(input).unwrap().1;

    let mut seen: HashSet<Coord3> = HashSet::new();
    let mut surface_area = 0isize;

    for &coord in drop.iter() {
        if seen.contains(&coord) {
            continue;
        }

        let mut queue = VecDeque::from([coord]);
        seen.insert(coord);

        while let Some(coord) = queue.pop_front() {
            for neighbour in NEIGHBOURS {
                let neighbour = coord + neighbour;
                if !drop.contains(&neighbour) {
                    surface_area += 1;
                    continue;
                }

                if seen.contains(&neighbour) {
                    continue;
                }

                seen.insert(neighbour);
                queue.push_back(neighbour);
            }
        }
    }

    surface_area
}

fn part2(input: &str) -> isize {
    let drop = parse_drop(input).unwrap().1;
    let (min, max) = bounding_box(&drop);
    let min = min + Coord3::new(-1, -1, -1);
    let max = max + Coord3::new(1, 1, 1);

    let mut seen: HashSet<Coord3> = HashSet::new();
    let mut surface_area = 0isize;

    let mut queue = VecDeque::from([min]);
    seen.insert(min);

    while let Some(coord) = queue.pop_front() {
        for neighbour in NEIGHBOURS {
            let neighbour = coord + neighbour;
            if neighbour.x < min.x
                || neighbour.y < min.y
                || neighbour.z < min.z
                || neighbour.x > max.x
                || neighbour.y > max.y
                || neighbour.z > max.z
            {
                continue;
            }

            if drop.contains(&neighbour) {
                surface_area += 1;
                continue;
            }

            if seen.contains(&neighbour) {
                continue;
            }

            seen.insert(neighbour);
            queue.push_back(neighbour);
        }
    }

    surface_area
}

fn main() {
    let input = include_str!("../in.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    const TINY_INPUT: &str = "1,1,1\n2,1,1,\n";
    const INPUT: &str = include_str!("../small-in.txt");

    #[test]
    fn part1_tiny() {
        assert_eq!(super::part1(TINY_INPUT), 10);
    }

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 64);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(INPUT), 58);
    }
}
