use cave::{Cave, DropResult};
use utils_22::Coord;

mod cave;

fn main() {
    let mut cave = Cave::from(include_str!("input"));

    let grains = (0..)
        .map_while(|_| match cave.drop_sand() {
            DropResult::Resting(pos) => Some(pos),
            DropResult::Overflow => None,
        })
        .count();

    println!("{}", grains);

    let mut cave = Cave::with_floor(include_str!("input"));
    let grains = (0..)
        .map_while(|_| match cave.drop_sand() {
            DropResult::Resting(pos) => {
                println!("{pos}");
                if pos == Coord::new(500, 0) {
                    None
                } else {
                    Some(pos)
                }
            }
            DropResult::Overflow => None,
        })
        .count()
        + 1;

    println!("{grains}");
}
