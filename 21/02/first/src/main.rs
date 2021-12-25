use utils::read_input_by_lines;

fn main() {
    let lines = read_input_by_lines()
        .filter_map(|l| {
            let mut split_line = l.split_whitespace();

            Some((split_line.next().unwrap().to_string(),
                  split_line.next().unwrap().parse::<i32>().unwrap()))
        }
        ).map(
        |(instr, dist)| {
            match instr.as_str() {
                "forward" => (dist, 0),
                "up" => (0, -dist),
                "down" => (0, dist),
                _ => panic!("FUCK")
            }
        }
    ).reduce(|(xa, ya), (xb, yb)| (xa + xb, ya + yb)).unwrap();

    println!("{}", lines.0 * lines.1);
    //.for_each(|(instr, dist)| println!("{}", instr));
}
