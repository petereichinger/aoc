use utils::read_by_lines;

mod packet;

fn main() {
    let line = read_by_lines("../input").next().unwrap();

    let packet = packet::Packet::from(line.as_str());

    println!("Version sum {}", packet.sum_versions());
    println!("Eval {}", packet.eval())
}