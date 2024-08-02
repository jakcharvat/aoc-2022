use std::{collections::HashSet, fmt::Display, io::stdin};

use advent::coord::Coord;

struct Visited<'a>(&'a HashSet<Coord>);
impl<'a> Display for Visited<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let (min, max) = self
            .0
            .iter()
            .fold((Coord::MAX, Coord::MIN), |(min, max), curr| {
                (min.min(*curr), max.max(*curr))
            });

        for y in min.y..=max.y {
            for x in min.x..=max.x {
                if self.0.contains(&Coord::new(x, y)) {
                    write!(f, "#")?;
                } else {
                    write!(f, ".")?;
                }
            }
            writeln!(f)?;
        }

        Ok(())
    }
}

fn main() {
    let mut rope = [Coord::zero(); 10];
    let mut knot_positions = rope.map(|coord| HashSet::from([coord]));

    let lines = stdin().lines().map(|l| l.unwrap());
    for line in lines {
        let step = match line.chars().nth(0).unwrap() {
            'L' => Coord::left(),
            'R' => Coord::right(),
            'U' => Coord::up(),
            'D' => Coord::down(),
            c => panic!(
                "Unknown direction char '{}' (ASCII {:?})",
                c,
                c.to_string().as_bytes()
            ),
        };

        let count: usize = line[2..].parse().unwrap();
        for _ in 0..count {
            rope[0] += step;
            knot_positions[0].insert(rope[0]);

            for i in 1..10usize {
                let body = rope[i - 1] - rope[i];
                if body.inf_norm() <= 1 {
                    break;
                }

                let move_by = Coord {
                    x: body.x.clamp(-1, 1),
                    y: body.y.clamp(-1, 1),
                };

                rope[i] += move_by;
                knot_positions[i].insert(rope[i]);
            }
        }
    }

    eprintln!("Part 1 movements: {}", Visited(&knot_positions[1]));
    eprintln!("Part 2 movements: {}", Visited(&knot_positions[9]));

    println!("Part 1: {}", knot_positions[1].len());
    println!("Part 2: {}", knot_positions[9].len());
}
