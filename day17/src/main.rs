use std::{
    array,
    collections::{HashMap, HashSet},
    fmt::Display,
};

use advent::coord::Coord;

const PRINT_DROPS: bool = false;

/*
 * All the different possible rock formations:
 *           ####
 *
 *           .#.
 *           ###
 *           .#.
 *
 *           ..#
 *           ..#
 *           ###
 *
 *           #
 *           #
 *           #
 *           #
 *
 *           ##
 *           ##
 */

#[derive(Clone, Copy)]
enum Rock {
    /// ####
    Minus,

    /// .#.
    /// ###
    /// .#.
    Plus,

    /// ..#
    /// ..#
    /// ###
    ArrowHead,

    /// #
    /// #
    /// #
    /// #
    I,

    /// ##
    /// ##
    Dot,
}

const CHAMBER_WIDTH: isize = 7;
const ROCKS_COUNT: usize = 2022;
const PART_2_ROCKS_COUNT: usize = 1000000000000;

#[repr(u64)]
#[derive(Clone, Copy)]
enum Direction {
    Up = 0b00,
    Down = 0b01,
    Left = 0b10,
    Right = 0b11,
}

impl Direction {
    fn turn_left(self) -> Direction {
        use Direction::*;

        match self {
            Up => Left,
            Down => Right,
            Left => Down,
            Right => Up,
        }
    }

    fn turn_right(self) -> Direction {
        use Direction::*;

        match self {
            Up => Right,
            Down => Left,
            Left => Up,
            Right => Down,
        }
    }

    fn step_coord(self) -> Coord {
        use Direction::*;

        match self {
            Up => Coord::down(),
            Down => Coord::up(),
            Left => Coord::left(),
            Right => Coord::right(),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Eq, Hash)]
struct Snapshot {
    rock_idx: usize,
    step_idx: usize,
    trace: u64,
}

impl Snapshot {
    fn trace(chamber: &Chamber) -> Option<u64> {
        let mut pos = Coord::new(0, chamber.highest_ys[0] + 1);
        let mut curr_dir = Direction::Right;
        let mut res = 0u64;

        let max_steps = std::mem::size_of_val(&res) * u8::BITS as usize / 2;
        for _ in 0..max_steps {
            if chamber.is_free(pos + curr_dir.turn_right().step_coord()) {
                // Try to turn right first
                curr_dir = curr_dir.turn_right();
            } else if chamber.is_free(pos + curr_dir.step_coord()) {
                // Else try to step forward (or we have reached the end)
            } else if chamber.is_free(pos + curr_dir.turn_left().step_coord()) {
                // Else try to turn left
                curr_dir = curr_dir.turn_left();
            } else {
                // Otherwise we must be able to go back
                assert!(chamber.is_free(pos + curr_dir.turn_left().turn_left().step_coord()));
                curr_dir = curr_dir.turn_left().turn_left();
            }

            pos = pos + curr_dir.step_coord();
            res = (res << 2) | curr_dir as u64;

            let last_x = CHAMBER_WIDTH - 1;
            if pos == Coord::new(last_x, chamber.highest_ys[last_x as usize] + 1) {
                return Some(res);
            }
        }

        None
    }

    fn take(chamber: &Chamber) -> Option<Snapshot> {
        Self::trace(chamber).map(|trace| Snapshot {
            rock_idx: chamber.rock_idx % Rock::all().len(),
            step_idx: chamber.step_idx % chamber.steps.len(),
            trace,
        })
    }
}

struct Chamber {
    chamber: [HashSet<isize>; CHAMBER_WIDTH as usize],
    highest_ys: [isize; CHAMBER_WIDTH as usize],
    steps: Vec<char>,
    step_idx: usize,
    rock_idx: usize,
}

impl Chamber {
    fn new(winds: &str) -> Chamber {
        Chamber {
            chamber: array::from_fn(|_| HashSet::new()),
            highest_ys: [-1; CHAMBER_WIDTH as usize],
            steps: winds.chars().collect(),
            step_idx: 0,
            rock_idx: 0,
        }
    }

    fn highest_y(&self) -> isize {
        *self.highest_ys.iter().max().unwrap()
    }

    fn is_free(&self, coord: Coord) -> bool {
        if coord.y < 0 || coord.x < 0 || coord.x >= CHAMBER_WIDTH {
            return false;
        }

        !self.chamber[coord.x as usize].contains(&coord.y)
    }

    fn fill_block(&mut self, coord: Coord) {
        assert!(
            self.chamber[coord.x as usize].insert(coord.y),
            "Attempting to add rock to already filled coord: {}",
            coord
        );

        self.highest_ys[coord.x as usize] = self.highest_ys[coord.x as usize].max(coord.y);
    }

    fn can_add_rock(&self, rock: Rock, origin_pos: Coord) -> bool {
        rock.coords_at(origin_pos).all(|c| self.is_free(c))
    }

    fn add_rock(&mut self, rock: Rock, origin_pos: Coord) {
        rock.coords_at(origin_pos)
            .for_each(|coord| self.fill_block(coord))
    }

    fn spawn_pos(&self) -> Coord {
        Coord::new(2, self.highest_y() + 4)
    }

    fn drop_rock(&mut self) {
        let rock = Rock::all()[self.rock_idx % Rock::all().len()];
        self.rock_idx += 1;

        let mut rock_pos = self.spawn_pos();
        if PRINT_DROPS {
            eprintln!(
                "Dropping rock {}:\n{}",
                self.rock_idx,
                ShowChamber(self, Some((rock, rock_pos)))
            );
        }

        loop {
            let step = self.steps[self.step_idx % self.steps.len()];
            self.step_idx += 1;

            let new_coord = match step {
                '>' => rock_pos + Coord::right(),
                '<' => rock_pos + Coord::left(),
                c => panic!("Unknown step '{}'", c),
            };

            if self.can_add_rock(rock, new_coord) {
                rock_pos = new_coord;
            }

            // Down is +1 y
            let new_coord = rock_pos + Coord::up();
            if self.can_add_rock(rock, new_coord) {
                rock_pos = new_coord;
            } else {
                self.add_rock(rock, rock_pos);
                break;
            }
        }
    }
}

impl Rock {
    fn all() -> &'static [Rock; 5] {
        use Rock::*;
        &[Minus, Plus, ArrowHead, I, Dot]
    }

    fn coords(&self) -> &'static [Coord] {
        macro_rules! coords {
            ( $( ($x:expr, $y:expr) ),* ) => {
                &[$( Coord { x: $x, y: $y } ),*]
            };
        }
        match self {
            Rock::Minus => coords![(0, 0), (1, 0), (2, 0), (3, 0)],
            Rock::Plus => coords![(1, 1), (0, 1), (1, 0), (2, 1), (1, 2)],
            Rock::ArrowHead => coords![(0, 0), (1, 0), (2, 0), (2, 1), (2, 2)],
            Rock::I => coords![(0, 0), (0, 1), (0, 2), (0, 3)],
            Rock::Dot => coords![(0, 0), (1, 0), (1, 1), (0, 1)],
        }
    }

    fn coords_at(&self, origin_pos: Coord) -> impl Iterator<Item = Coord> {
        self.coords().iter().map(move |c| *c + origin_pos)
    }
}

struct ShowChamber<'a>(&'a Chamber, Option<(Rock, Coord)>);
impl<'a> Display for ShowChamber<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let max_chamber_y = self.0.highest_y();
        let rock_coords = self
            .1
            .map(|(rock, coord)| rock.coords_at(coord).collect::<Vec<_>>());

        let max_rock_y = rock_coords
            .as_ref()
            .map(|coords| coords.iter().map(|c| c.y).max().unwrap())
            .unwrap_or(max_chamber_y);

        let max_y = max_chamber_y.max(max_rock_y);

        for y in (0..=max_y).rev() {
            write!(f, "|")?;
            for x in 0..CHAMBER_WIDTH {
                if rock_coords
                    .as_ref()
                    .filter(|coords| coords.contains(&Coord::new(x, y)))
                    .is_some()
                {
                    write!(f, "@")?;
                } else if self.0.is_free(Coord::new(x, y)) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }

            writeln!(f, "|")?;
        }

        writeln!(f, "+-------+")
    }
}

fn part1(input: &str) -> isize {
    let mut chamber = Chamber::new(input);

    for _ in 0..ROCKS_COUNT {
        chamber.drop_rock();
    }

    chamber.highest_y() + 1
}

fn part2(input: &str) -> isize {
    let mut chamber = Chamber::new(input);
    let mut snapshots: HashMap<Snapshot, (isize, usize)> = HashMap::new();

    let (rocks_in_loop, loop_height) = loop {
        chamber.drop_rock();
        let Some(snapshot) = Snapshot::take(&chamber) else {
            continue;
        };

        if snapshots.contains_key(&snapshot) {
            break (
                chamber.rock_idx - snapshots[&snapshot].1,
                chamber.highest_y() - snapshots[&snapshot].0,
            );
        }

        snapshots.insert(snapshot, (chamber.highest_y(), chamber.rock_idx));
    };

    let remaining_drops = PART_2_ROCKS_COUNT - chamber.rock_idx;
    let loops = remaining_drops / rocks_in_loop;
    let left_to_sim = remaining_drops % rocks_in_loop;

    let looped_height = loop_height * loops as isize;
    for _ in 0..left_to_sim {
        chamber.drop_rock();
    }

    chamber.highest_y() + looped_height + 1
}

fn main() {
    let input = include_str!("../in.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

/*

.......
.##....
..#....
#.#....

Up = 0b00,
Down = 0b01,
Left = 0b10,
Right = 0b11,

*/

#[cfg(test)]
mod tests {
    use super::*;

    const INPUT: &str = include_str!("../small-in.txt");

    #[test]
    fn test_tracing() {
        let mut ch = Chamber::new("");

        macro_rules! fill {
            ( $( ($x:expr, $y:expr) ),* ) => {
                $( fill!($x, $y); )*
            };

            ( $x:expr, $y:expr ) => {
                ch.fill_block(Coord::new::<isize>($x, $y))
            };
        }

        assert_eq!(Snapshot::trace(&ch), Some(0b11_11_11_11_11_11));

        fill!(0, 0);
        assert_eq!(Snapshot::trace(&ch), Some(0b11_01_11_11_11_11_11));

        /* .......
         * .##....
         * ..#....
         * #.#.... */
        fill![(2, 0), (2, 1), (2, 2), (1, 2)];
        assert_eq!(
            Snapshot::trace(&ch),
            Some(0b11_01_00_10_00_00_11_11_11_01_01_01_11_11_11)
        );

        /* .......
         * #......
         * .##....
         * ..#....
         * #.#.... */
        fill!(0, 3);
        assert_eq!(Snapshot::trace(&ch), Some(0b11_01_11_11_01_01_01_11_11_11));
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(INPUT), 3068);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(INPUT), 1514285714288);
    }
}
