use std::{
    fmt::Display,
    ops::{Add, AddAssign, Sub, SubAssign},
};

#[derive(Default, Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Coord {
    pub x: i32,
    pub y: i32,
}

pub const ZERO: Coord = Coord { x: 0, y: 0 };
pub const UP: Coord = Coord { x: 0, y: 1 };
pub const DOWN: Coord = Coord { x: 0, y: -1 };
pub const LEFT: Coord = Coord { x: -1, y: 0 };
pub const RIGHT: Coord = Coord { x: 1, y: 0 };

fn diff(a: i32, b: i32) -> u32 {
    (a - b).unsigned_abs()
}

fn normalize(a: i32) -> i32 {
    if a == 0 {
        a
    } else {
        a / a.abs()
    }
}

impl Coord {
    pub fn distance_to(&self, other: &Self) -> u32 {
        let x_diff = diff(self.x, other.x);
        let y_diff = diff(self.y, other.y);

        u32::max(x_diff, y_diff)
    }

    pub fn limit_to_neighbour(&self) -> Self {
        let x = normalize(self.x);
        let y = normalize(self.y);

        Coord { x, y }
    }
}

impl Add<Coord> for Coord {
    type Output = Coord;

    fn add(self, rhs: Coord) -> Self::Output {
        Coord {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign<Coord> for Coord {
    fn add_assign(&mut self, rhs: Coord) {
        *self = *self + rhs;
    }
}

impl Sub<Coord> for Coord {
    type Output = Coord;

    fn sub(self, rhs: Coord) -> Self::Output {
        Coord {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign<Coord> for Coord {
    fn sub_assign(&mut self, rhs: Coord) {
        *self = *self - rhs;
    }
}

impl Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}
