use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, Mul, Sub, SubAssign},
};

type CoordT = isize;

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq)]
pub struct Coord {
    pub x: CoordT,
    pub y: CoordT,
}

impl Coord {
    pub fn new(x: CoordT, y: CoordT) -> Coord {
        Coord { x, y }
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

impl Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {})", self.x, self.y)
    }
}
