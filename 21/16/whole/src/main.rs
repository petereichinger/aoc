use utils::read_by_lines;

mod packet;

fn main() {
    let line = read_by_lines("../literal").next().unwrap();

    let packet = packet::Packet::from(line.as_str());

    println!("{:?}", packet);
}