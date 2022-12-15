mod tunnels;

use tunnels::Tunnels;
use utils_22::Coord;
const INPUT: &str = include_str!("input");
fn main() {
    part1();
    part2();
}
fn part1() {
    let tunnels = Tunnels::from(INPUT);

    let sensors = &tunnels.sensors;

    println!("{:#?}", sensors);
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
    for coord in (min_x..=max_x).map(|x| Coord::new(x, 2000000)) {
        for sensor in sensors {
            if coord.manhattan(&sensor.coord) <= sensor.manhattan {
                posses.push(coord);
                break;
            }
        }
    }

    posses.retain(|pos| sensors.iter().map(|s| s.closest_beacon).all(|b| pos != &b));

    println!("Part 1 {}", posses.len())
}

fn part2() {
    let tunnels = Tunnels::from(INPUT);

    let distress_beacon = tunnels.find_distress_beacon().expect("found not beacon");

    println!(
        "{}",
        (distress_beacon.x() as i64) * 4000000i64 + distress_beacon.y() as i64
    );
}
