use std::{array, io::stdin};

fn parse_trees() -> Vec<Vec<u32>> {
    stdin()
        .lines()
        .map(|l| {
            l.unwrap()
                .chars()
                .map(|c| c.to_digit(10).unwrap())
                .collect()
        })
        .collect()
}

fn visibility_matrix(trees: &Vec<Vec<u32>>) -> Vec<Vec<bool>> {
    let mut visible = vec![vec![false; trees[0].len()]; trees.len()];

    for line in visible.iter_mut() {
        *line.first_mut().unwrap() = true;
        *line.last_mut().unwrap() = true;
    }

    visible.first_mut().unwrap().fill(true);
    visible.last_mut().unwrap().fill(true);

    visible
}

type Coord = (usize, usize);

macro_rules! row {
    ( $v:expr, $y:expr ) => {
        $v[$y].iter().enumerate().map(move |(x, el)| (($y, x), el))
    };
}

macro_rules! rows {
    ( $v:expr ) => {
        (0..$v.len()).map(|y| row!($v, y))
    };
}

macro_rules! col {
    ( $v:expr, $x:expr ) => {
        $v.iter()
            .enumerate()
            .map(move |(y, line)| ((y, $x), &line[$x]))
    };
}

macro_rules! cols {
    ( $v:expr ) => {
        (0..$v[0].len()).map(|x| col!($v, x))
    };
}

fn look<'a>(mut trees: impl Iterator<Item = (Coord, &'a u32)>, visible: &mut Vec<Vec<bool>>) {
    let mut max_height = *trees.next().unwrap().1;
    for ((y, x), &height) in trees {
        if height > max_height {
            visible[y][x] = true;
            max_height = height;

            if max_height == 9 {
                break;
            }
        }
    }
}

fn fill_view_distances<'a>(
    trees: impl Iterator<Item = (Coord, &'a u32)>,
    distances: &mut Vec<Vec<usize>>,
) {
    let mut last_seen = [0usize; 10];
    for (idx, ((y, x), &tree)) in trees.enumerate() {
        distances[y][x] = idx - last_seen[tree as usize];

        for height in 0..=tree {
            last_seen[height as usize] = idx;
        }
    }
}

fn main() {
    let trees = parse_trees();
    let mut visible = visibility_matrix(&trees);

    for row in rows!(trees) {
        look(row.clone(), &mut visible);
        look(row.rev(), &mut visible);
    }

    for col in cols!(trees) {
        look(col.clone(), &mut visible);
        look(col.rev(), &mut visible);
    }

    println!(
        "Part 1: {:?}",
        visible
            .iter()
            .map(|l| l.iter().filter(|v| **v).count())
            .sum::<usize>()
    );

    let mut distances: [_; 4] = array::from_fn(|_| vec![vec![0usize; trees[0].len()]; trees.len()]);
    for row in rows!(trees) {
        fill_view_distances(row.clone(), &mut distances[0]);
        fill_view_distances(row.rev(), &mut distances[1]);
    }

    for col in cols!(trees) {
        fill_view_distances(col.clone(), &mut distances[2]);
        fill_view_distances(col.rev(), &mut distances[3]);
    }

    let [mut distances, rest @ ..] = distances;
    for other in rest {
        for (dist_line, other_line) in distances.iter_mut().zip(other) {
            for (dist_el, other_el) in dist_line.iter_mut().zip(other_line) {
                *dist_el *= other_el;
            }
        }
    }

    println!("Part 2: {}", distances.into_iter().flatten().max().unwrap())
}
