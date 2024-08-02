use std::{
    collections::{HashMap, VecDeque},
    io::stdin,
};

use advent::{coord::Coord, vec2d};

fn find_char_in_maze(maze: &Vec<String>, search_byte: u8) -> Option<Coord> {
    maze.iter()
        .enumerate()
        .find_map(|(y, line)| {
            line.as_bytes()
                .iter()
                .enumerate()
                .find_map(|(x, b)| Some(x).filter(|_| *b == search_byte))
                .map(move |x| (y, x))
        })
        .map(|(y, x)| Coord::new(x as isize, y as isize))
}

fn print_journeys(prev: &Vec<Vec<Option<Coord>>>) {
    for (y, line) in prev.iter().enumerate() {
        for (x, prev) in line.iter().enumerate() {
            let curr = Coord::new(x as isize, y as isize);
            let Some(prev) = prev else {
                print!(".");
                continue;
            };

            let step = curr - *prev;
            print!(
                "{}",
                match step {
                    x if x == Coord::zero() => 'o',
                    x if x == Coord::up() => 'v',
                    x if x == Coord::down() => '^',
                    x if x == Coord::left() => '>',
                    x if x == Coord::right() => '<',
                    x => panic!("Impossible step: {}", x),
                }
            )
        }

        println!();
    }
}

fn search<End>(maze: &Vec<Vec<u8>>, start_pos: Coord, mut is_end: End) -> i32
where
    End: FnMut(Coord) -> bool,
{
    let mut queue = VecDeque::from([(start_pos, 0)]);
    let mut prev = vec2d![None as Option<Coord>; maze];
    prev[start_pos] = Some(start_pos);

    eprintln!("searching from {}", start_pos);
    let dist = 'outer: loop {
        let (curr, dist) = queue.pop_front().unwrap_or_else(|| {
            print_journeys(&prev);
            panic!("No solution");
        });

        for step in Coord::left_down_right_up() {
            let new_coord = curr + step;
            let new_dist = dist + 1;

            if !new_coord.is_in_grid(&maze)
                || prev[new_coord].is_some()
                || maze[curr] > maze[new_coord] + 1
            {
                continue;
            }

            if is_end(new_coord) {
                break 'outer new_dist;
            }

            queue.push_back((new_coord, new_dist));
            prev[new_coord] = Some(curr);
        }
    };

    print_journeys(&prev);
    dist
}

fn main() {
    let maze: Vec<_> = stdin().lines().map(|l| l.unwrap()).collect();
    let start_pos = find_char_in_maze(&maze, b'S').unwrap();
    let end_pos = find_char_in_maze(&maze, b'E').unwrap();

    let mut maze: Vec<_> = maze.into_iter().map(|line| line.into_bytes()).collect();
    maze[start_pos] = b'a';
    maze[end_pos] = b'z';
    let maze = maze;

    println!(
        "Part 1: {}",
        search(&maze, end_pos, |coord| coord == start_pos)
    );

    println!(
        "Part 2: {}",
        search(&maze, end_pos, |coord| maze[coord] == b'a')
    )
}
