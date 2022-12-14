use std::{
    fmt::Display,
    ops::{Add, AddAssign, Sub, SubAssign},
};

use crate::LineIterator;

#[derive(Default, Debug, Copy, Clone, Hash, PartialEq, Eq)]
pub struct Coord {
    x: i32,
    y: i32,
}

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
    pub const ZERO: Coord = Coord { x: 0, y: 0 };
    pub const UP: Coord = Coord { x: 0, y: 1 };
    pub const DOWN: Coord = Coord { x: 0, y: -1 };
    pub const LEFT: Coord = Coord { x: -1, y: 0 };
    pub const RIGHT: Coord = Coord { x: 1, y: 0 };

    pub const ORTHOGONAL_NEIGHBOURS: [Coord; 4] =
        [Coord::UP, Coord::RIGHT, Coord::DOWN, Coord::LEFT];

    pub fn new(x: i32, y: i32) -> Self {
        Coord { x, y }
    }

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

    pub fn x(&self) -> i32 {
        self.x
    }

    pub fn y(&self) -> i32 {
        self.y
    }

    pub fn manhattan(&self, other: &Self) -> u32 {
        (self.x - other.x).unsigned_abs() + (self.y - other.y).unsigned_abs()
    }

    pub fn dist(&self, other: &Self) -> f32 {
        let x = (self.x - other.x).abs() as f32;
        let y = (self.y - other.y).abs() as f32;

        f32::sqrt(x * x + y * y)
    }

    pub fn line_to(&self, other: &Self) -> LineIterator {
        LineIterator::new(self, other)
    }

    pub fn normalized(&self) -> Self {
        assert!(self.x == 0 || self.y == 0);

        Coord {
            x: normalize(self.x),
            y: normalize(self.y),
        }
    }

    pub fn max(&self, other: &Self) -> Self {
        Self {
            x: self.x.max(other.x),
            y: self.y.max(other.y),
        }
    }

    pub fn min(&self, other: &Self) -> Self {
        Self {
            x: self.x.min(other.x),
            y: self.y.min(other.y),
        }
    }
}

impl Add<&Coord> for &Coord {
    type Output = Coord;

    fn add(self, rhs: &Coord) -> Self::Output {
        Coord {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
        }
    }
}

impl AddAssign<&Coord> for Coord {
    fn add_assign(&mut self, rhs: &Coord) {
        *self = &*self + rhs;
    }
}

impl Sub<&Coord> for &Coord {
    type Output = Coord;

    fn sub(self, rhs: &Coord) -> Self::Output {
        Coord {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
        }
    }
}

impl SubAssign<&Coord> for Coord {
    fn sub_assign(&mut self, rhs: &Coord) {
        *self = &*self - rhs;
    }
}

impl Display for Coord {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "({},{})", self.x, self.y)
    }
}

impl From<&str> for Coord {
    fn from(input: &str) -> Self {
        let (x, y) = input.split_once(',').unwrap();
        let x = x.parse::<i32>().unwrap();
        let y = y.parse::<i32>().unwrap();
        Coord { x, y }
    }
}
