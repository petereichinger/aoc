#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Shape {
    pub width: usize,
    pub height: usize,
    pub rows: Vec<Vec<bool>>,
}

pub fn get_shapes_iter() -> impl Iterator<Item = Shape> {
    let shapes = vec![
        Shape {
            width: 4,
            height: 1,
            rows: vec![vec![true, true, true, true]],
        },
        Shape {
            width: 3,
            height: 3,
            rows: vec![
                vec![false, true, false],
                vec![true, true, true],
                vec![false, true, false],
            ],
        },
        Shape {
            width: 3,
            height: 3,
            rows: vec![
                vec![true, true, true],
                vec![false, false, true],
                vec![false, false, true],
            ],
        },
        Shape {
            width: 1,
            height: 4,
            rows: vec![vec![true], vec![true], vec![true], vec![true]],
        },
        Shape {
            width: 2,
            height: 2,
            rows: vec![vec![true, true], vec![true, true]],
        },
    ];

    shapes.into_iter().cycle()
}

#[cfg(test)]
mod tests {
    use super::get_shapes_iter;

    #[test]
    fn check_repeats() {
        let shapes: Vec<_> = get_shapes_iter().take(10).collect();

        (0..5)
            .into_iter()
            .for_each(|idx| assert_eq!(shapes[idx], shapes[idx + 5]));
    }
}
