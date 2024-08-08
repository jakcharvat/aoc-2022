use std::array;

#[allow(unused_imports)]
use dbg_pls::pretty;
use parser::{parse_blueprints, Blueprint, Materials};

mod parser;

fn optimistic_estimate(materials: &Materials, robots: &Materials, mins: u32) -> u32 {
    let curr_geo = get!(geo materials);
    let curr_geobots = get!(geo robots) * mins;
    let new_geobots = (1 + mins) * mins / 2;

    curr_geo + curr_geobots + new_geobots
}

fn simulate_blueprint_rec(
    mins: u32,
    bp: &Blueprint,
    materials: Materials,
    robots: Materials,
    mut best_solution: u32,
) -> u32 {
    if mins == 0 {
        return get!(geo materials);
    }

    if optimistic_estimate(&materials, &robots, mins) <= best_solution {
        return u32::MIN;
    }

    let max_robots: Materials = array::from_fn(|i| bp.iter().map(|r| r[i]).max().unwrap());
    best_solution = best_solution.max(get!(geo materials) + mins * get!(geo robots));

    'next_recipe: for (robot_idx, recipe) in bp.iter().enumerate().rev() {
        if robot_idx < 3 && robots[robot_idx] >= max_robots[robot_idx] {
            continue;
        }

        let mut time_to_collect = 0;
        for (material_idx, &count) in recipe.iter().enumerate() {
            if count == 0 {
                continue;
            }

            if robots[material_idx] == 0 {
                continue 'next_recipe;
            }

            let needed = count.checked_sub(materials[material_idx]).unwrap_or(0);
            time_to_collect =
                time_to_collect.max((needed + robots[material_idx] - 1) / robots[material_idx]);
        }

        if time_to_collect + 1 > mins {
            continue;
        }

        let new_materials: Materials =
            array::from_fn(|i| materials[i] + (time_to_collect + 1) * robots[i] - recipe[i]);
        let mut new_robots = robots.clone();
        new_robots[robot_idx] += 1;

        best_solution = best_solution.max(simulate_blueprint_rec(
            mins - time_to_collect - 1,
            bp,
            new_materials,
            new_robots,
            best_solution,
        ));
    }

    best_solution
}

fn simulate_blueprint(bp: &Blueprint, mins: u32) -> u32 {
    simulate_blueprint_rec(mins, bp, [0; 4], [1, 0, 0, 0], u32::MIN)
}

fn part1(input: &str) -> usize {
    let blueprints = parse_blueprints(input).unwrap().1;
    blueprints
        .iter()
        .enumerate()
        .map(|(idx, bp)| (idx + 1) * simulate_blueprint(bp, 24) as usize)
        .sum::<usize>()
}

fn part2(input: &str) -> usize {
    let blueprints = parse_blueprints(input).unwrap().1;
    blueprints
        .iter()
        .take(3)
        .map(|bp| simulate_blueprint(bp, 32) as usize)
        .product()
}

fn main() {
    let input = include_str!("../in.txt");
    println!("Part 1: {}", part1(input));
    println!("Part 2: {}", part2(input));
}

#[cfg(test)]
mod tests {
    const INPUT: &str = include_str!("../small-in.txt");

    #[test]
    fn part1() {
        assert_eq!(super::part1(INPUT), 33);
    }
}
