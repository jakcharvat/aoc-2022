use crate::coord::Coord;

#[derive(Clone, Copy, Debug, dbg_pls::DebugPls)]
pub struct Bounds {
    pub min: Coord,
    pub max: Coord,
}

impl Bounds {
    pub fn new() -> Bounds {
        Bounds {
            min: Coord::MAX,
            max: Coord::MIN,
        }
    }

    fn with(self, coord: &Coord) -> Bounds {
        Bounds {
            min: self.min.min(*coord),
            max: self.max.max(*coord),
        }
    }
}

trait Internal {}

#[allow(private_bounds)]
pub trait CoordBounded: Internal {
    fn coord_bounds(self) -> Bounds;
}

impl<'a, I: Iterator<Item = &'a Coord>> Internal for I {}
impl<'a, I: Iterator<Item = &'a Coord>> CoordBounded for I {
    fn coord_bounds(self) -> Bounds {
        self.fold(Bounds::new(), |bounds, curr| bounds.with(curr))
    }
}
