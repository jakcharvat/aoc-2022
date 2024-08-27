use std::{collections::HashSet, fmt::Display};

use advent::{bounds::CoordBounded, coord::Coord};

use crate::SAND_DROP;

pub struct Sandbox<'a> {
    pub grid: &'a HashSet<Coord>,
    pub sandbox: &'a HashSet<Coord>,
    pub has_floor: bool,
}

fn print_header(
    f: &mut std::fmt::Formatter<'_>,
    min_x: isize,
    max_x: isize,
    leading_padding: usize,
) -> std::fmt::Result {
    write!(f, "{:width$} ", "", width = leading_padding as usize)?;
    write!(
        f,
        "{:-<width$}",
        "",
        width = (SAND_DROP.x - min_x).max(0) as usize
    )?;
    write!(f, "v")?;
    writeln!(
        f,
        "{:-<width$}",
        "",
        width = (max_x - SAND_DROP.x).max(0) as usize
    )?;

    Ok(())
}

impl<'a> Display for Sandbox<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let grid_bounds = self.grid.iter().coord_bounds();
        let sand_bounds = self.sandbox.iter().coord_bounds();
        let min = grid_bounds.min.min(sand_bounds.min);
        let max = grid_bounds.max.max(sand_bounds.max);
        assert!(min.y >= 0);

        let y_label_width = (max.y + 2).to_string().len();
        print_header(f, min.x - 1, max.x + 1, y_label_width)?;

        for y in 0..=(max.y + 1) {
            write!(f, "{:>width$} ", y, width = y_label_width)?;
            for x in (min.x - 1)..=(max.x + 1) {
                let curr = Coord::new(x, y);
                let ch = match self.grid.contains(&curr) {
                    true => '█',
                    false => match self.sandbox.contains(&curr) {
                        true => 'o',
                        false => '.',
                    },
                };

                write!(f, "{}", ch)?;
            }

            writeln!(f)?;
        }

        if self.has_floor {
            write!(f, "{:>width$} ", max.y + 2, width = y_label_width)?;
            write!(f, "{:█<width$}", "", width = (max.x - min.x + 3) as usize)?;
        }

        Ok(())
    }
}

#[macro_export]
macro_rules! grid {
    ( $grid:expr ) => {
        Sandbox {
            grid: &$grid,
            sandbox: &$grid,
            has_floor: false,
        }
    };

    ( $grid:expr, floor ) => {
        Sandbox {
            grid: &$grid,
            sandbox: &$grid,
            has_floor: true,
        }
    };
}

#[macro_export]
macro_rules! sandbox {
    ( $grid:expr, $sandbox:expr ) => {
        Sandbox {
            grid: &$grid,
            sandbox: &$sandbox,
            has_floor: false,
        }
    };

    ( $grid:expr, $sandbox:expr, floor ) => {
        Sandbox {
            grid: &$grid,
            sandbox: &$sandbox,
            has_floor: true,
        }
    };
}
