use utils_22::Coord;

#[derive(Debug)]
pub struct Sensor {
    pub coord: Coord,
    pub closest_beacon: Coord,
    pub manhattan: u32,
}

pub struct Tunnels {
    pub sensors: Vec<Sensor>,
}

impl Tunnels {
    pub fn find_distress_beacon(&self) -> Option<Coord> {
        let sensors = &self.sensors;

        const ROWS: i32 = 4000000;

        for row in 0..=ROWS {
            let mut ranges = sensors
                .iter()
                .filter_map(|sensor| {
                    let delta_row = (row - sensor.coord.y()).abs();
                    let width = sensor.manhattan as i32 - delta_row;
                    if width < 0 {
                        return None;
                    }

                    Some((sensor.coord.x() - width, sensor.coord.x() + width))
                })
                .collect::<Vec<_>>();

            ranges.sort_by(|(a_min, a_max), (b_min, b_max)| {
                a_min.cmp(&b_min).then(a_max.cmp(&b_max))
            });

            // #[cfg(test)]
            // if row == 11 {
            //     println!("{:#?}", ranges);
            // }

            let mut invalid_x = 0;

            for (begin, end) in &ranges {
                if begin > &invalid_x {
                    return Some(Coord::new(invalid_x, row));
                }

                invalid_x = (end + 1).max(invalid_x);
            }
        }

        None
    }
}

fn get_coord_from_str(input: &str) -> Coord {
    // getting coords in format x=%, y=%

    if let Some((x, y)) = input.split_once(", ") {
        let x = x.strip_prefix("x=").unwrap().parse::<i32>().unwrap();
        let y = y.strip_prefix("y=").unwrap().parse::<i32>().unwrap();

        Coord::new(x, y)
    } else {
        panic!("invalid coord input {}", input)
    }
}

impl From<&str> for Tunnels {
    fn from(input: &str) -> Self {
        let sensors = input
            .lines()
            .map(|line| {
                if let Some((sensor, beacon)) = line
                    .strip_prefix("Sensor at ")
                    .unwrap()
                    .split_once(": closest beacon is at ")
                {
                    let sensor = get_coord_from_str(sensor);
                    let beacon = get_coord_from_str(beacon);
                    let manhattan = sensor.manhattan(&beacon);
                    Sensor {
                        coord: sensor,
                        closest_beacon: beacon,
                        manhattan,
                    }
                } else {
                    panic!("invalid line {}", line)
                }
            })
            .collect();

        Tunnels { sensors }
    }
}

#[cfg(test)]
mod tests {
    use utils_22::Coord;

    use super::Tunnels;

    const TEST: &str = include_str!("test");
    const INPUT: &str = include_str!("input");
    #[test]
    fn parsing_works() {
        let _ = Tunnels::from(TEST);
        let _ = Tunnels::from(INPUT);
    }

    #[test]
    fn test_data() {
        let tunnels = Tunnels::from(TEST);
        let sensors = &tunnels.sensors;
        let min_x = sensors
            .iter()
            .map(|s| s.coord.x() - s.manhattan as i32)
            .min()
            .unwrap();
        let max_x = tunnels
            .sensors
            .iter()
            .map(|s| s.coord.x() + s.manhattan as i32)
            .max()
            .unwrap();

        let mut posses = Vec::new();
        for coord in (min_x..=max_x).map(|x| Coord::new(x, 10)) {
            for sensor in sensors {
                if coord.manhattan(&sensor.coord) <= sensor.manhattan {
                    posses.push(coord);
                    break;
                }
            }
        }

        posses.retain(|pos| sensors.iter().map(|s| s.closest_beacon).all(|b| pos != &b));

        assert_eq!(posses.len(), 26);
    }

    #[test]
    fn test_distress_beacon() {
        let tunnels = Tunnels::from(TEST);

        let pos = tunnels.find_distress_beacon();

        assert_eq!(pos, Some(Coord::new(14, 11)));
    }
}
