#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Direction {
    Left,
    Right,
}

pub fn get_direction_iter(input: &str) -> impl Iterator<Item = Direction> + '_ {
    input
        .chars()
        .map(|c| match c {
            '<' => Direction::Left,
            '>' => Direction::Right,
            _ => panic!("invalid char {} detected", c),
        })
        .cycle()
}

#[cfg(test)]
mod tests {
    use crate::direction_stream::Direction;

    use super::get_direction_iter;

    #[test]
    fn simple_test() {
        let stream: Vec<_> = get_direction_iter("<>").take(2).collect();

        assert_eq!(stream, vec![Direction::Left, Direction::Right]);
    }

    #[test]
    fn repat_test() {
        let stream: Vec<_> = get_direction_iter("<>").take(4).collect();

        assert_eq!(
            stream,
            vec![
                Direction::Left,
                Direction::Right,
                Direction::Left,
                Direction::Right
            ]
        );
    }
}
