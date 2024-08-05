use std::{collections::HashSet, env::args, io::stdin, thread};

use advent::coord::Coord;
use dbg_pls::pretty;
use parser::{parse_sensor, SensorData};
mod parser;

fn cover_ranges(data: &Vec<SensorData>, y: isize) -> Vec<(isize, isize)> {
    let mut ranges: Vec<_> = data
        .iter()
        .filter_map(|sensor| {
            let dist = (sensor.sensor_pos - sensor.beacon_pos).manhattan();
            let y_dist = (sensor.sensor_pos.y - y).abs();

            if y_dist > dist {
                return None;
            }

            let x_span = dist - y_dist;
            let mut left = sensor.sensor_pos.x - x_span;
            let mut right = sensor.sensor_pos.x + x_span;

            assert_eq!((Coord::new(left, y) - sensor.sensor_pos).manhattan(), dist);
            assert_eq!((Coord::new(right, y) - sensor.sensor_pos).manhattan(), dist);

            if sensor.beacon_pos.y == y {
                if left == sensor.beacon_pos.x {
                    left += 1;
                } else if right == sensor.beacon_pos.x {
                    right -= 1;
                } else {
                    unreachable!()
                }
            }

            Some((left, right))
        })
        .collect();

    ranges.sort();
    ranges
}

fn count_covered_spots(data: &Vec<SensorData>, y: isize) -> isize {
    let ranges = cover_ranges(data, y);

    let mut rightmost_x = isize::MIN;
    let mut count = 0isize;

    for (from, to) in ranges.iter().copied() {
        if from > rightmost_x {
            count += to - from + 1;
            rightmost_x = to;
        } else if to > rightmost_x {
            count += to - rightmost_x;
            rightmost_x = to;
        }
    }

    count
}

fn find_available_spot_in_line(
    data: &Vec<SensorData>,
    y: isize,
    xmin: isize,
    xmax: isize,
) -> Option<Coord> {
    let ranges = cover_ranges(data, y);

    let mut rightmost_x = xmin - 1;
    let mut spot = None;

    for (from, to) in ranges {
        if from > xmax {
            break;
        }

        if from > rightmost_x {
            if from == rightmost_x + 2 {
                assert_eq!(spot, None);
                spot = Some(from - 1);
            } else {
                assert!(from < rightmost_x + 2, "{} < {}", from, rightmost_x + 2)
            }

            rightmost_x = to;
        } else if to > rightmost_x {
            rightmost_x = to;
        }
    }

    spot.map(|x| Coord::new(x, y))
}

fn find_available_spot_in_bounds(data: Vec<SensorData>, min: Coord, max: Coord) -> Option<Coord> {
    let beacons: HashSet<Coord> = data.iter().map(|s| s.beacon_pos).collect();
    let spots: Vec<_> = (min.y..=max.y)
        .filter_map(|y| find_available_spot_in_line(&data, y, min.x, max.x))
        .filter(|pos| !beacons.contains(pos))
        .collect();

    assert!(spots.len() <= 1, "Spots: {:?}", spots);
    spots.into_iter().next()
}

fn main() {
    let y = args()
        .skip(1)
        .next()
        .expect("y position (10 for sample input, 2000000 for big input)")
        .parse::<isize>()
        .unwrap();

    let bound = y * 2;

    let sensors: Vec<_> = stdin()
        .lines()
        .map(|l| parse_sensor(&l.unwrap()).unwrap().1)
        .collect();

    println!("Part 1: {}", count_covered_spots(&sensors, y));

    let threads = std::thread::available_parallelism()
        .unwrap()
        .min((bound as usize / 5).try_into().unwrap())
        .get() as isize;

    eprintln!("Running part two on {} threads...", threads);

    let count = (bound + 1) / threads + 1;
    let threads: Vec<_> = (0..threads)
        .map(|i| {
            let s = sensors.clone();

            thread::spawn(move || {
                let from = i * count;
                let to = bound.min(from + count - 1);

                find_available_spot_in_bounds(s, Coord::new(0, from), Coord::new(bound, to))
            })
        })
        .collect();

    let spots: Vec<_> = threads
        .into_iter()
        .filter_map(|t| t.join().unwrap())
        .collect();

    pretty!(&spots);
    assert_eq!(spots.len(), 1);

    println!("Part 2: {}", spots[0].x * 4000000 + spots[0].y)
}
