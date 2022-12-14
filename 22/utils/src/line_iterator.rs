use super::Coord;

pub struct LineIterator {
    current: Coord,
    end: Coord,
    dir: Coord,
}

impl LineIterator {
    pub fn new(start: &Coord, end: &Coord) -> Self {
        let diff = end - start;

        if diff == Coord::ZERO {
            return LineIterator {
                current: *start,
                end: end + &Coord::RIGHT,
                dir: Coord::RIGHT,
            };
        }

        assert!(diff.x() == 0 || diff.y() == 0);

        let dir = diff.normalized();

        LineIterator {
            current: *start,
            end: end + &dir,
            dir,
        }
    }
}

impl Iterator for LineIterator {
    type Item = Coord;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current == self.end {
            None
        } else {
            let ret_val = self.current;
            self.current += &self.dir;
            Some(ret_val)
        }
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        let diff = &self.end - &self.current;
        let remaining = (diff.x().max(diff.y())) as usize;
        (remaining, Some(remaining))
    }
}

#[cfg(test)]
mod tests {
    use crate::{Coord, LineIterator};

    #[test]
    fn iteration_works() {
        let line_iter = LineIterator::new(&Coord::ZERO, &Coord::new(2, 0));
        let coords: Vec<Coord> = line_iter.collect();

        assert_eq!(
            coords,
            vec![Coord::new(0, 0), Coord::new(1, 0), Coord::new(2, 0)]
        )
    }

    #[test]
    fn point_works() {
        let line_iter = LineIterator::new(&Coord::ZERO, &Coord::ZERO);
        let coords: Vec<Coord> = line_iter.collect();

        assert_eq!(coords, vec![Coord::new(0, 0)])
    }

    #[test]
    #[should_panic]
    fn diagonal_fails() {
        LineIterator::new(&Coord::ZERO, &Coord::new(1, 1));
    }
}
