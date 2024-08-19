use std::{
    collections::{hash_map::Entry, HashMap, HashSet},
    fmt::Display,
    iter::from_fn,
    usize,
};

use advent::{bounds::CoordBounded, coord::Coord};
use strum::EnumCount;

#[allow(unused_imports)]
use dbg_pls::pretty;

fn input() -> &'static str {
    if cfg!(test) {
        include_str!("../small-in.txt")
    } else {
        include_str!("../in.txt")
    }
}

#[derive(Clone, Copy, Debug, EnumCount, strum::FromRepr)]
#[repr(u8)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn clockwise(self) -> Direction {
        Self::from_repr((self as u8 + 1) % Self::COUNT as u8).unwrap()
    }

    fn anticlockwise(self) -> Direction {
        Self::from_repr((self as u8 + 3) % Self::COUNT as u8).unwrap()
    }

    fn step(self) -> Coord {
        match self {
            Direction::North => Coord::up(),
            Direction::East => Coord::right(),
            Direction::South => Coord::down(),
            Direction::West => Coord::left(),
        }
    }

    fn move_preferences() -> impl Iterator<Item = impl Iterator<Item = Direction> + Clone> {
        use Direction::*;

        let mut prefs = [North, South, West, East].into_iter().cycle();
        from_fn(move || {
            let it = prefs.clone();
            prefs.next().unwrap();
            Some(it.take(4))
        })
    }
}

#[derive(dbg_pls::DebugPls)]
enum Proposition {
    Single(Coord),
    Multiple,
}

struct ShowMap<'a>(&'a HashSet<Coord>);
impl<'a> Display for ShowMap<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let bb = self.0.iter().coord_bounds();

        for y in bb.min.y..=bb.max.y {
            for x in bb.min.x..=bb.max.x {
                write!(
                    f,
                    "{}",
                    match self.0.contains(&Coord::new(x, y)) {
                        true => '#',
                        false => '.',
                    }
                )?;
            }

            writeln!(f)?;
        }

        Ok(())
    }
}

trait CoordNeighbour {
    fn neighbour(self, directions: &[Direction]) -> Self;
}

impl CoordNeighbour for Coord {
    fn neighbour(self, directions: &[Direction]) -> Self {
        self + directions.into_iter().map(|d| d.step()).sum::<Coord>()
    }
}

fn simulate(rounds: Option<usize>) -> isize {
    let lines: Vec<&[u8]> = input().lines().map(|l| l.as_bytes()).collect();
    let mut map = HashSet::new();
    for (y, line) in lines.iter().enumerate() {
        for (x, &ch) in line.iter().enumerate() {
            if ch == b'#' {
                map.insert(Coord::new(x as isize, y as isize));
            }
        }
    }

    if rounds.is_some() {
        eprintln!("{}", ShowMap(&map));
    }

    for (round, preferences) in Direction::move_preferences()
        .take(rounds.unwrap_or(usize::MAX))
        .enumerate()
    {
        let mut propositions = HashMap::<Coord, Proposition>::new();

        for &coord in map.iter() {
            let is_empty = |dirs: &[Direction]| !map.contains(&coord.neighbour(dirs));

            if is_empty(&[Direction::North])
                && is_empty(&[Direction::North, Direction::East])
                && is_empty(&[Direction::East])
                && is_empty(&[Direction::South, Direction::East])
                && is_empty(&[Direction::South])
                && is_empty(&[Direction::South, Direction::West])
                && is_empty(&[Direction::West])
                && is_empty(&[Direction::North, Direction::West])
            {
                continue;
            }

            let Some(proposed_dir) = preferences.clone().find(|&dir| {
                is_empty(&[dir])
                    && is_empty(&[dir, dir.clockwise()])
                    && is_empty(&[dir, dir.anticlockwise()])
            }) else {
                continue;
            };

            match propositions.entry(coord + proposed_dir.step()) {
                Entry::Vacant(entry) => _ = entry.insert(Proposition::Single(coord)),
                Entry::Occupied(mut entry) => *entry.get_mut() = Proposition::Multiple,
            };
        }

        let mut move_propositions = propositions
            .into_iter()
            .filter_map(|(to, prop)| match prop {
                Proposition::Single(from) => Some((from, to)),
                Proposition::Multiple => None,
            })
            .peekable();

        if rounds.is_none() && move_propositions.peek().is_none() {
            return round as isize + 1;
        }

        for (from, to) in move_propositions {
            assert!(map.remove(&from));
            assert!(map.insert(to));
        }

        if rounds.is_some() {
            eprintln!("{}", ShowMap(&map));
        }
    }

    let bb = map.iter().coord_bounds();
    (bb.max.y - bb.min.y + 1) * (bb.max.x - bb.min.x + 1) - map.len() as isize
}

fn part1() -> isize {
    simulate(Some(10))
}

fn part2() -> isize {
    simulate(None)
}

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        assert_eq!(super::part1(), 110)
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(), 20)
    }
}
