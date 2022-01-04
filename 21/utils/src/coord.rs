use std::fmt::{Debug, Display, Formatter};
use std::ops::{Add, AddAssign, Sub};

#[derive(PartialEq, Debug, Copy, Clone, Hash, Eq)]
pub struct Coord {
    x: i32,
    y: i32,
}

#[derive(PartialEq, Debug, Copy, Clone)]
pub struct Line {
    start: Coord,
    end: Coord,
}

impl Line {
    pub fn from_coords(start: Coord, end: Coord) -> Line {
        Line { start, end }
    }

    pub fn orthogonal(&self) -> bool {
        self.start.x == self.end.x || self.start.y == self.end.y
    }
}

pub struct LineInterpolator {
    current: Coord,
    direction: Coord,
    end: Coord,
}


impl LineInterpolator {
    pub fn new(line: &Line) -> LineInterpolator {
        let diff = line.end - line.start;

        let x_dir = if diff.x != 0 { diff.x / diff.x.abs() } else { 0 };
        let y_dir = if diff.y != 0 { diff.y / diff.y.abs() } else { 0 };

        let direction = Coord { x: x_dir, y: y_dir };

        LineInterpolator { current: line.start, direction, end: line.end + direction }
    }
}

impl Iterator for LineInterpolator {
    type Item = Coord;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.end {
            None
        } else {
            let ret_val = self.current;
            self.current += self.direction;
            Some(ret_val)
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

impl From<&Line> for LineInterpolator {
    fn from(line: &Line) -> Self {
        LineInterpolator::new(line)
    }
}

fn parse_value(coord: &str) -> i32 {
    coord.parse().unwrap()
}

impl From<&str> for Coord {
    fn from(value: &str) -> Self {
        let (x, y) = value.split_once(',').unwrap();
        Coord {
            x: parse_value(x),
            y: parse_value(y),
        }
    }
}

impl Display for Coord {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}

impl Display for Line {
    fn fmt(&self, f: &mut Formatter<'_>) -> std::fmt::Result {
        Debug::fmt(self, f)
    }
}