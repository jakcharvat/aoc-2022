use std::{
    collections::HashSet,
    io::{stdin, Read},
};

use advent::coord::{Coord, CoordUtils};
use parser::parse_lines;
use sandbox_drawer::Sandbox;

mod parser;
mod sandbox_drawer;

const SAND_DROP: Coord = Coord { x: 500, y: 0 };

fn clamp(coord: Coord, min: isize, max: isize) -> Coord {
    Coord {
        x: coord.x.clamp(min, max),
        y: coord.y.clamp(min, max),
    }
}

fn make_grid(lines: &Vec<Vec<Coord>>) -> HashSet<Coord> {
    let mut set = HashSet::new();
    for line in lines {
        for (&from, &to) in line.iter().zip(line.iter().skip(1)) {
            let mut coord = from;
            let step = clamp(to - from, -1, 1);

            while coord != to {
                set.insert(coord);
                coord += step;
            }

            set.insert(coord);
        }
    }

    set
}

fn drop_sand(sandbox: &mut HashSet<Coord>, max_y: isize) -> bool {
    let mut pos = SAND_DROP;
    loop {
        pos = if !sandbox.contains(&(pos + Coord::down())) {
            pos + Coord::down()
        } else if !sandbox.contains(&(pos + Coord::down() + Coord::left())) {
            pos + Coord::down() + Coord::left()
        } else if !sandbox.contains(&(pos + Coord::down() + Coord::right())) {
            pos + Coord::down() + Coord::right()
        } else {
            break;
        };

        if pos.y > max_y {
            return false;
        }
    }

    sandbox.insert(pos);
    true
}

fn drop_sand_with_floor(sandbox: &mut HashSet<Coord>, max_y: isize) -> bool {
    if sandbox.contains(&SAND_DROP) {
        return false;
    }

    let mut pos = SAND_DROP;
    loop {
        pos = if !sandbox.contains(&(pos + Coord::down())) {
            pos + Coord::down()
        } else if !sandbox.contains(&(pos + Coord::down() + Coord::left())) {
            pos + Coord::down() + Coord::left()
        } else if !sandbox.contains(&(pos + Coord::down() + Coord::right())) {
            pos + Coord::down() + Coord::right()
        } else {
            break;
        };

        if pos.y >= max_y + 1 {
            break;
        }
    }

    sandbox.insert(pos);
    true
}

fn main() {
    let print_iterations = false;

    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let (input, lines) = parse_lines(&input).unwrap();
    assert!(input.trim().is_empty());

    let grid = make_grid(&lines);
    let max_y = grid.iter().coord_bounds().max().y;
    eprintln!("Original grid:\n{}", grid!(grid));

    let mut sandbox = grid.clone();
    let mut counter = 0..;
    while drop_sand(&mut sandbox, max_y) {
        if print_iterations {
            eprintln!(
                "Iteration {}:\n{}",
                counter.next().unwrap(),
                sandbox!(grid, sandbox)
            )
        }
    }

    eprintln!("Final sandbox:\n{}", sandbox!(grid, sandbox));
    println!("Part 1: {}", sandbox.len() - grid.len());

    eprintln!("Original grid with floor:\n{}", grid!(grid, floor));

    let mut sandbox = grid.clone();
    let mut counter = 0..;
    while drop_sand_with_floor(&mut sandbox, max_y) {
        if print_iterations {
            eprintln!(
                "Iteration {}:\n{}",
                counter.next().unwrap(),
                sandbox!(grid, sandbox, floor)
            )
        }
    }

    eprintln!("Final sandbox:\n{}", sandbox!(grid, sandbox, floor));
    println!("Part 2: {}", sandbox.len() - grid.len());
}
