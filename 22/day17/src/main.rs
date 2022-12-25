mod direction_stream;
mod game;
mod shapes;

use game::Game;

fn main() {
    part1();
    part2();
}

fn part1() {
    const INPUT: &str = include_str!("input");

    let mut game = Game::new(INPUT);

    for _ in 0..2022 {
        game.drop_rock();
    }

    println!("Part 1 {}", game.height);
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct State {
    x: usize,
}

fn part2() {
    // const INPUT: &str = include_str!("input");
    // let mut game = Game::new(INPUT);

    // let mut states = vec![];
    // let pos = game.drop_rock();

    // game.drop_rock();

    // loop {
    //     game.drop_rock();

    //     for prefix_len in 0..=game.drops - 2 {}
    // }
}
