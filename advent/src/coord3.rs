use std::{
    fmt::Display,
    ops::{Add, AddAssign, Div, Mul, Sub, SubAssign},
};

type CoordT = isize;

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, dbg_pls::DebugPls)]
pub struct Coord3 {
    pub x: CoordT,
    pub y: CoordT,
    pub z: CoordT,
}

impl Coord3 {
    pub fn new(x: isize, y: isize, z: isize) -> Coord3 {
        Coord3 { x, y, z }
    }

    pub fn zero() -> Coord3 {
        Coord3 { x: 0, y: 0, z: 0 }
    }
}

impl Add for Coord3 {
    type Output = Coord3;

    fn add(self, rhs: Self) -> Self::Output {
        Coord3 {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl AddAssign for Coord3 {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
    }
}

impl Sub for Coord3 {
    type Output = Coord3;

    fn sub(self, rhs: Self) -> Self::Output {
        Coord3 {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl SubAssign for Coord3 {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
    }
}

impl Mul<CoordT> for Coord3 {
    type Output = Coord3;

    fn mul(self, rhs: CoordT) -> Self::Output {
        Coord3 {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Mul<Coord3> for CoordT {
    type Output = Coord3;

    fn mul(self, rhs: Coord3) -> Self::Output {
        Coord3 {
            x: self * rhs.x,
            y: self * rhs.y,
            z: self * rhs.z,
        }
    }
}

impl Div<CoordT> for Coord3 {
    type Output = Coord3;

    fn div(self, rhs: CoordT) -> Self::Output {
        Coord3 {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Div<Coord3> for CoordT {
    type Output = Coord3;

    fn div(self, rhs: Coord3) -> Self::Output {
        Coord3 {
            x: self / rhs.x,
            y: self / rhs.y,
            z: self / rhs.z,
        }
    }
}

impl Display for Coord3 {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({}, {}, {})", self.x, self.y, self.z)
    }
}
