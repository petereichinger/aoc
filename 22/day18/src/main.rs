mod droplet;

use droplet::Droplet;

const INPUT: &str = include_str!("input");

fn main() {
    let droplet: Droplet = INPUT.into();

    println!("Part 1 {}", droplet.get_total_surface_area());
    println!("Part 2 {}", droplet.get_outer_surface_area());
}
