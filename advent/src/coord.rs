use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, Index, IndexMut, Mul, Sub, SubAssign},
};

type CoordT = isize;

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, dbg_pls::DebugPls)]
pub struct Coord {
    pub x: CoordT,
    pub y: CoordT,
}

impl Coord {
    pub fn new<T>(x: T, y: T) -> Coord
    where
        T: Into<CoordT>,
    {
        Coord {
            x: x.into(),
            y: y.into(),
        }
    }

    pub fn zero() -> Coord {
        Coord { x: 0, y: 0 }
    }

    pub fn left() -> Coord {
        Coord { x: -1, y: 0 }
    }

    pub fn right() -> Coord {
        Coord { x: 1, y: 0 }
    }

    pub fn up() -> Coord {
        Coord { x: 0, y: -1 }
    }

    pub fn down() -> Coord {
        Coord { x: 0, y: 1 }
    }

    pub fn left_down_right_up() -> [Coord; 4] {
        [Coord::left(), Coord::down(), Coord::right(), Coord::up()]
    }

    pub fn inf_norm(&self) -> CoordT {
        self.x.abs().max(self.y.abs())
    }

    pub const MIN: Coord = Coord {
        x: CoordT::MIN,
        y: CoordT::MIN,
    };

    pub const MAX: Coord = Coord {
        x: CoordT::MAX,
        y: CoordT::MAX,
    };

    pub fn min(self, rhs: Coord) -> Coord {
        Coord {
            x: self.x.min(rhs.x),
            y: self.y.min(rhs.y),
        }
    }

    pub fn max(self, rhs: Coord) -> Coord {
        Coord {
            x: self.x.max(rhs.x),
            y: self.y.max(rhs.y),
        }
    }

    pub fn is_in_grid<T>(&self, grid: &Vec<Vec<T>>) -> bool {
        self.y >= 0
            && (self.y as usize) < grid.len()
            && self.x >= 0
            && (self.x as usize) < grid[0].len()
    }

    pub fn manhattan(&self) -> isize {
        self.x.abs() + self.y.abs()
    }
}

impl Add for Coord {
    type Output = Coord;

    fn add(self, rhs: Self) -> Self::Output {
        Coord {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign for Coord {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
    }
}

impl Sub for Coord {
    type Output = Coord;

    fn sub(self, rhs: Self) -> Self::Output {
        Coord {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign for Coord {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
    }
}

impl Mul<CoordT> for Coord {
    type Output = Coord;

    fn mul(self, rhs: CoordT) -> Self::Output {
        Coord {
            x: self.x * rhs,
            y: self.y * rhs,
        }
    }
}

impl Mul<Coord> for CoordT {
    type Output = Coord;

    fn mul(self, rhs: Coord) -> Self::Output {
        Coord {
            x: self * rhs.x,
            y: self * rhs.y,
        }
    }
}

impl Div<CoordT> for Coord {
    type Output = Coord;

    fn div(self, rhs: CoordT) -> Self::Output {
        Coord {
            x: self.x / rhs,
            y: self.y / rhs,
        }
    }
}

impl Div<Coord> for CoordT {
    type Output = Coord;

    fn div(self, rhs: Coord) -> Self::Output {
        Coord {
            x: self / rhs.x,
            y: self / rhs.y,
        }
    }
}

impl<T> Index<Coord> for Vec<Vec<T>> {
    type Output = T;

    fn index(&self, index: Coord) -> &Self::Output {
        let y: usize = index.y.try_into().unwrap();
        let x: usize = index.x.try_into().unwrap();
        &self[y][x]
    }
}

impl<T> IndexMut<Coord> for Vec<Vec<T>> {
    fn index_mut(&mut self, index: Coord) -> &mut Self::Output {
        let y: usize = index.y.try_into().unwrap();
        let x: usize = index.x.try_into().unwrap();
        &mut self[y][x]
    }
}

impl Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
