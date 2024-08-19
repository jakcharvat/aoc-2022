use advent::coord::{ContainsCoord, Coord};
#[allow(unused_imports)]
use dbg_pls::pretty;

mod part2;

fn parse_instruction_line(input: &str) -> Vec<Vec<usize>> {
    input
        .split('R')
        .map(|chunk| chunk.split('L').map(|len| len.parse().unwrap()).collect())
        .collect()
}

#[derive(Clone, Copy, PartialEq, Eq, dbg_pls::DebugPls, Debug)]
enum Field {
    OutOfMap,
    Empty,
    Wall,
}

#[derive(Clone, Copy, PartialEq, Eq, Debug)]
#[repr(u8)]
enum Direction {
    North,
    East,
    South,
    West,
}

impl Direction {
    fn from(discriminant: u8) -> Direction {
        match discriminant {
            0 => Self::North,
            1 => Self::East,
            2 => Self::South,
            3 => Self::West,
            _ => panic!("Invalid direction: {}", discriminant),
        }
    }

    fn clockwise(self) -> Direction {
        Self::from((self as u8 + 1) % 4)
    }

    fn anticlockwise(self) -> Direction {
        Self::from((self as u8 + 3) % 4)
    }

    fn turn_right(self) -> Direction {
        self.clockwise()
    }

    fn turn_left(self) -> Direction {
        self.anticlockwise()
    }

    fn opposite(self) -> Direction {
        self.clockwise().clockwise()
    }

    fn step(self) -> Coord {
        match self {
            Direction::North => Coord::up(),
            Direction::South => Coord::down(),
            Direction::West => Coord::left(),
            Direction::East => Coord::right(),
        }
    }

    const CLOCKWISE: [Direction; 4] = [
        Direction::North,
        Direction::East,
        Direction::South,
        Direction::West,
    ];
}

fn split_grid_and_instruction(input: &str) -> (Vec<&str>, &str) {
    let mut lines = input.lines().collect::<Vec<_>>();
    let separator_line_idx = lines.len() - 2;
    assert!(lines[separator_line_idx].is_empty());

    let instructions = lines.pop().unwrap();
    _ = lines.pop().unwrap();

    (lines, instructions)
}

fn make_grid(grid_lines: &Vec<&str>) -> Vec<Vec<Field>> {
    let grid_height = grid_lines.len();
    let grid_width = grid_lines.iter().map(|l| l.len()).max().unwrap();
    let mut grid = vec![vec![Field::OutOfMap; grid_width]; grid_height];

    for (y, line) in grid_lines.iter().enumerate() {
        for (x, field) in line.chars().enumerate() {
            grid[y][x] = match field {
                ' ' => Field::OutOfMap,
                '.' => Field::Empty,
                '#' => Field::Wall,
                c => panic!("unknown field character {}", c),
            };
        }
    }

    grid
}

fn calc_coordinate(pos: Coord, dir: Direction) -> isize {
    (pos.y + 1) * 1000
        + (pos.x + 1) * 4
        + match dir {
            Direction::North => 3,
            Direction::South => 1,
            Direction::West => 2,
            Direction::East => 0,
        }
}

fn part1(input: &str) -> isize {
    let (grid_lines, instruction_line) = split_grid_and_instruction(input);

    let grid = make_grid(&grid_lines);
    let instructions = parse_instruction_line(instruction_line);

    let starting_x = grid[0].iter().position(|&f| f != Field::OutOfMap).unwrap();
    let mut curr_pos = Coord::new(starting_x as isize, 0);
    let mut curr_dir = Direction::East;

    for (right_step_idx, right_steps) in instructions.iter().enumerate() {
        if right_step_idx != 0 {
            curr_dir = curr_dir.turn_right();
        }

        for (left_step_idx, &left_steps) in right_steps.iter().enumerate() {
            if left_step_idx != 0 {
                curr_dir = curr_dir.turn_left();
            }

            for _ in 0..left_steps {
                let next_pos = curr_pos + curr_dir.step();
                let next_field = if grid.contains_coord(next_pos) {
                    grid[next_pos]
                } else {
                    Field::OutOfMap
                };

                curr_pos = match next_field {
                    Field::OutOfMap => {
                        let opp_dir = curr_dir.turn_left().turn_left();
                        let mut pos = curr_pos;
                        while grid.contains_coord(pos + opp_dir.step())
                            && grid[pos + opp_dir.step()] != Field::OutOfMap
                        {
                            pos += opp_dir.step();
                        }

                        match grid[pos] {
                            Field::OutOfMap => unreachable!(),
                            Field::Empty => pos,
                            Field::Wall => break,
                        }
                    }
                    Field::Empty => curr_pos + curr_dir.step(),
                    Field::Wall => break,
                };
            }
        }
    }

    calc_coordinate(curr_pos, curr_dir)
}

fn main() {
    let input = include_str!("../in.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2::part2(input));
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("../small-in.txt");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 6032);
    }

    #[test]
    fn part2() {
        assert_eq!(super::part2::part2(INPUT), 5031);
    }
}
