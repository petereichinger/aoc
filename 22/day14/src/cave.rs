use utils_22::Coord;

pub struct Cave {
    max: Coord,
}

impl From<&str> for Cave {
    fn from(input: &str) -> Self {
        for line in input.lines() {
            let coords: Vec<Coord> = line.split(" -> ").map(|str| Coord::from(str)).collect();

            for pair in coords.windows(2) {
                let begin = &pair[0];
                let end = &pair[1];

                for coord in begin.line_to(end) {}
            }
        }

        todo!()
    }
}
