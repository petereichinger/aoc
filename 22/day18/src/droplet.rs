use std::ops::Add;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Cell {
    Inside,
    Lava,
    Outside,
}

#[derive(Debug, Clone, Copy)]
pub struct Coord(isize, isize, isize);

const ORTHO_NEIGHBOURS: [Coord; 6] = [
    Coord(-1, 0, 0),
    Coord(1, 0, 0),
    Coord(0, -1, 0),
    Coord(0, 1, 0),
    Coord(0, 0, -1),
    Coord(0, 0, 1),
];

const NEIGHBOURS: [Coord; 26] = [
    // lower
    Coord(-1, -1, -1),
    Coord(0, -1, -1),
    Coord(1, -1, -1),
    Coord(-1, -1, 0),
    Coord(0, -1, 0),
    Coord(1, -1, 0),
    Coord(-1, -1, 1),
    Coord(0, -1, 1),
    Coord(1, -1, 1),
    // same height
    Coord(-1, 0, -1),
    Coord(0, 0, -1),
    Coord(1, 0, -1),
    Coord(-1, 0, 0),
    Coord(1, 0, 0),
    Coord(-1, 0, 1),
    Coord(0, 0, 1),
    Coord(1, 0, 1),
    //higher
    Coord(-1, 1, -1),
    Coord(0, 1, -1),
    Coord(1, 1, -1),
    Coord(-1, 1, 0),
    Coord(0, 1, 0),
    Coord(1, 1, 0),
    Coord(-1, 1, 1),
    Coord(0, 1, 1),
    Coord(1, 1, 1),
];

pub struct Droplet {
    cells: Box<[Cell]>,
    width: isize,
    height: isize,
    depth: isize,
}

impl Droplet {
    fn from_coordinates(coordinates: Vec<Coord>) -> Self {
        let max = coordinates
            .iter()
            .fold(Coord(0, 0, 0), |Coord(mx, my, mz), Coord(ex, ey, ez)| {
                Coord(mx.max(*ex), my.max(*ey), mz.max(*ez))
            });

        let width = max.0 + 1;
        let height = max.1 + 1;
        let depth = max.2 + 1;
        let vec = vec![Cell::Inside; (width * height * depth) as usize];

        let mut droplet = Self {
            cells: vec.into_boxed_slice(),
            width,
            height,
            depth,
        };

        for coordinate in coordinates {
            let cell = droplet
                .get_cell_mut(coordinate)
                .expect("coord out of range during construction");

            *cell = Cell::Lava;
        }

        let mut stack = vec![Coord(0, 0, 0)];

        while let Some(c) = stack.pop() {
            if let Some(cell) = droplet.get_cell_mut(c) {
                *cell = Cell::Outside;
                ORTHO_NEIGHBOURS
                    .iter()
                    .map(|n| Coord(c.0 + n.0, c.1 + n.1, c.2 + n.2))
                    .for_each(|n| match droplet.get_cell(n) {
                        Some(&Cell::Inside) => stack.push(n),
                        _ => {}
                    });
            }
        }

        droplet
    }

    pub fn coord_to_usize(&self, coord: Coord) -> Option<isize> {
        if coord.0 < 0 || coord.1 < 0 || coord.2 < 0 {
            return None;
        }

        if coord.0 > self.width || coord.1 > self.height || coord.2 > self.depth {
            return None;
        }

        Some(coord.1 * self.width * self.depth + coord.2 * self.width + coord.0)
    }

    pub fn get_cell(&self, coord: Coord) -> Option<&Cell> {
        let index = self.coord_to_usize(coord)?;

        self.cells.get(index as usize)
    }

    pub fn get_cell_mut(&mut self, coord: Coord) -> Option<&mut Cell> {
        let index = self.coord_to_usize(coord)?;

        self.cells.get_mut(index as usize)
    }

    fn get_surface_area(
        &self,
        check_fn: fn(cell: &Cell, neighbour: Option<&Cell>) -> Option<()>,
    ) -> usize {
        let mut surface_area = 0;

        for y in 0..self.height {
            for z in 0..self.depth {
                for x in 0..self.width {
                    let coord = Coord(x, y, z);
                    let cell = self.get_cell(coord).unwrap();
                    if let Some(&Cell::Lava) = self.get_cell(coord) {
                        surface_area += ORTHO_NEIGHBOURS
                            .iter()
                            .map(|Coord(nx, ny, nz)| Coord(nx + x, ny + y, nz + z))
                            .filter_map(|c| check_fn(cell, self.get_cell(c)))
                            .count();
                    }
                }
            }
        }
        surface_area
    }

    fn total_check(_cell: &Cell, neighbour: Option<&Cell>) -> Option<()> {
        if let Some(Cell::Lava) = neighbour {
            None
        } else {
            Some(())
        }
    }

    fn outer_check(_cell: &Cell, neighbour: Option<&Cell>) -> Option<()> {
        match neighbour {
            Some(&Cell::Outside) | None => Some(()),
            _ => None,
        }
    }

    pub fn get_total_surface_area(&self) -> usize {
        self.get_surface_area(Droplet::total_check)
    }

    pub fn get_outer_surface_area(&self) -> usize {
        self.get_surface_area(Droplet::outer_check)
    }
}

impl From<&str> for Droplet {
    fn from(input: &str) -> Self {
        let coordinates: Vec<_> = input
            .lines()
            .map(|l| {
                let mut split = l.split(',');
                let x = split.next().unwrap().parse::<_>().unwrap();
                let y = split.next().unwrap().parse::<_>().unwrap();
                let z = split.next().unwrap().parse::<_>().unwrap();

                Coord(x, y, z)
            })
            .collect();

        Droplet::from_coordinates(coordinates)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    const TEST: &str = include_str!("test");

    #[test]
    fn creating_works() {
        let _droplet: Droplet = TEST.into();
    }

    #[test]
    fn simple_works() {
        let droplet: Droplet = "1,1,1\n2,1,1".into();

        assert_eq!(droplet.width, 3);
        assert_eq!(droplet.height, 2);
        assert_eq!(droplet.depth, 2);

        assert_eq!(droplet.coord_to_usize(Coord(1, 1, 1)), Some(10));
        assert_eq!(droplet.coord_to_usize(Coord(2, 1, 1)), Some(11));
    }

    #[test]
    fn lava_cell_set_correctly() {
        let droplet: Droplet = "1,1,1\n2,1,1".into();

        assert_eq!(droplet.get_cell(Coord(1, 1, 1)), Some(&Cell::Lava))
    }

    #[test]
    fn Inside_cell_set_correctly() {
        let droplet: Droplet = "1,1,1\n2,1,1".into();

        assert_eq!(droplet.get_cell(Coord(0, 0, 0)), Some(&Cell::Inside))
    }

    #[test]
    fn get_simple_surface_area() {
        let droplet: Droplet = "1,1,1\n2,1,1".into();

        assert_eq!(droplet.get_total_surface_area(), 10);
    }

    #[test]
    fn get_single_surface_area() {
        let droplet: Droplet = "1,1,1".into();

        assert_eq!(droplet.get_total_surface_area(), 6);
    }

    #[test]
    fn get_star_surface_area() {
        let droplet: Droplet = "2,1,2
2,2,1
1,2,2
2,2,2
3,2,2
2,2,3
2,3,2"
            .into();

        assert_eq!(droplet.get_total_surface_area(), 30);
    }

    #[test]
    fn get_zero_surface_area() {
        let droplet: Droplet = "0,0,0".into();

        assert_eq!(droplet.get_total_surface_area(), 6);
    }

    #[test]
    fn get_test_surface_area() {
        let droplet: Droplet = TEST.into();

        assert_eq!(droplet.get_total_surface_area(), 64);
    }

    #[test]
    fn get_outer_surface_area() {
        let droplet: Droplet = TEST.into();

        let inside_cells: Vec<_> = droplet
            .cells
            .iter()
            .enumerate()
            .filter(|(_idx, c)| matches!(c, &Cell::Inside))
            .collect();

        assert_eq!(inside_cells.len(), 1);

        let outer_surface_area = droplet.get_outer_surface_area();

        assert_eq!(outer_surface_area, 58);
    }
}
