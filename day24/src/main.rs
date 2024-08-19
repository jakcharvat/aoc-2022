use std::{
    cell::RefCell,
    collections::{HashMap, VecDeque},
    fmt::Display,
    ops::Index,
};

use advent::coord::{ContainsCoord, Coord};
#[allow(unused_imports)]
use dbg_pls::pretty;
use strum::{EnumCount, FromRepr, VariantArray};

#[cfg(test)]
const INPUT: &str = include_str!("../small-in.txt");
#[cfg(not(test))]
const INPUT: &str = include_str!("../in.txt");

#[repr(u8)]
#[derive(Clone, Copy, Debug, EnumCount, FromRepr, VariantArray)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn step(self) -> Coord {
        match self {
            Direction::North => Coord::up(),
            Direction::East => Coord::right(),
            Direction::South => Coord::down(),
            Direction::West => Coord::left(),
        }
    }

    fn char(self) -> char {
        match self {
            Direction::North => '^',
            Direction::East => '>',
            Direction::South => 'v',
            Direction::West => '<',
        }
    }
}

#[derive(Clone, Debug, dbg_pls::DebugPls)]
struct Blizzards {
    blizzards: [Vec<Vec<bool>>; Direction::COUNT],

    width: usize,
    height: usize,

    curr_time: usize,
}

impl Blizzards {
    fn new() -> Blizzards {
        let lines: Vec<&[u8]> = INPUT.lines().map(|l| l.as_bytes()).collect();

        let height = lines.len() - 2;
        let width = lines[0].len() - 2;

        for line in lines.iter() {
            assert!(line.len() == width + 2);
        }

        Self::check_top_and_bottom_row(&lines, width, height);

        let mut blizzards = std::array::from_fn(|_| vec![vec![false; width]; height]);

        for y in 0..height {
            assert_eq!(lines[y][0], b'#');
            assert_eq!(lines[y][width + 1], b'#');

            for x in 0..width {
                let dir = match lines[y + 1][x + 1] {
                    b'^' => Direction::North,
                    b'>' => Direction::East,
                    b'v' => Direction::South,
                    b'<' => Direction::West,
                    b'.' => continue,
                    c => panic!("Invalid map char: {} ({:?})", c, c),
                };

                blizzards[dir as usize][y][x] = true;
            }
        }

        Blizzards {
            blizzards,
            width,
            height,
            curr_time: 0,
        }
    }

    fn check_top_and_bottom_row(lines: &[&[u8]], w: usize, h: usize) {
        for (x, &ch) in lines[0].iter().enumerate() {
            if x == 1 {
                assert_eq!(ch, b'.');
            } else {
                assert_eq!(ch, b'#');
            }
        }

        for (x, &ch) in lines[h + 1].iter().enumerate() {
            if x == w {
                assert_eq!(ch, b'.');
            } else {
                assert_eq!(ch, b'#');
            }
        }
    }
}

impl Blizzards {
    fn set_time(&mut self, time: usize) {
        self.curr_time = time;
    }

    fn direction_blizzard(&self, coord: Coord, dir: Direction) -> bool {
        let pos = coord - dir.step() * self.curr_time as isize;
        let mod_pos = Coord::new(
            pos.x.rem_euclid(self.width as isize),
            pos.y.rem_euclid(self.height as isize),
        );

        self.blizzards[dir as usize][mod_pos]
    }

    fn contains_coord(&self, coord: Coord) -> bool {
        self.blizzards[0].contains_coord(coord)
    }
}

impl Index<Coord> for Blizzards {
    type Output = bool;

    fn index(&self, index: Coord) -> &Self::Output {
        match Direction::VARIANTS
            .iter()
            .any(|&dir| self.direction_blizzard(index, dir))
        {
            true => &true,
            false => &false,
        }
    }
}

impl Display for Blizzards {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "#.{:#<width$}", "", width = self.width)?;
        for y in 0..self.height {
            write!(f, "#")?;
            for x in 0..self.width {
                let coord = Coord::new(x as isize, y as isize);
                let winds: Vec<bool> = Direction::VARIANTS
                    .into_iter()
                    .map(|&dir| self.direction_blizzard(coord, dir))
                    .collect();

                let count = winds.iter().filter(|&&w| w).count();
                let ch = match count {
                    0 => '.',
                    1 => {
                        let dir_idx = winds.iter().position(|&w| w).unwrap();
                        let dir = Direction::from_repr(dir_idx as u8).unwrap();
                        dir.char()
                    }
                    c @ 2..=9 => (b'0' + c as u8) as char,
                    c => panic!("Cannot have {} blizzards in one spot!", c),
                };

                write!(f, "{}", ch)?;
            }

            writeln!(f, "#")?;
        }
        writeln!(f, "{:#<width$}.#", "", width = self.width)
    }
}

fn run(blizzards: &mut Blizzards, start: Coord, end: Coord, start_time: usize) -> usize {
    let blizzards = RefCell::new(blizzards);
    let queue = RefCell::new(VecDeque::new());
    let seen = RefCell::new(HashMap::<Coord, usize>::new());

    let enqueue = |coord: Coord, time: usize| {
        if coord != start && !blizzards.borrow().contains_coord(coord) {
            return None;
        }

        if coord != start && blizzards.borrow()[coord]
            || seen
                .borrow()
                .get(&coord)
                .filter(|&&seen| seen >= time)
                .is_some()
        {
            return None;
        }

        if coord == end {
            return Some(time + 1);
        }

        seen.borrow_mut().insert(coord, time);
        queue.borrow_mut().push_back((coord, time));

        None
    };

    assert!(enqueue(start, start_time).is_none());

    loop {
        let Some((curr_coord, curr_time)) = queue.borrow_mut().pop_front() else {
            unreachable!("The puzzle doesn't have a solution")
        };

        let new_time = curr_time + 1;
        blizzards.borrow_mut().set_time(new_time);

        for dir in Direction::VARIANTS {
            if let Some(time) = enqueue(curr_coord + dir.step(), new_time) {
                return time;
            };
        }

        if let Some(time) = enqueue(curr_coord, new_time) {
            return time;
        }
    }
}

pub fn part1() -> usize {
    let mut blizzards = Blizzards::new();

    let start = Direction::North.step();
    let end = Coord::new(blizzards.width as isize - 1, blizzards.height as isize - 1);

    run(&mut blizzards, start, end, 0)
}

pub fn part2() -> usize {
    let mut blizzards = Blizzards::new();

    let there_start = Direction::North.step();
    let there_end = Coord::new(blizzards.width as isize - 1, blizzards.height as isize - 1);

    let back_start = there_end + Direction::South.step();
    let back_end = Coord::zero();

    let there = run(&mut blizzards, there_start, there_end, 0);
    eprintln!("There in {}", there);
    let back = run(&mut blizzards, back_start, back_end, there);
    eprintln!("Back in {}", back);
    run(&mut blizzards, there_start, there_end, back)
}

fn main() {
    println!("Part 1: {}", part1());
    println!("Part 2: {}", part2());
}

#[cfg(test)]
mod tests {
    #[test]
    fn part1() {
        assert_eq!(super::part1(), 18);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2(), 54);
    }
}
