use std::cell::RefCell;

use crate::monkey::Monkey;

pub fn get_monkeys() -> Vec<RefCell<Monkey>> {
    vec![
        // Monkey 0:
        //   Starting items: 71, 86
        //   Operation: new = old * 13
        //   Test: divisible by 19
        //     If true: throw to monkey 6
        //     If false: throw to monkey 7
        Monkey {
            id: 0,
            items: vec![71, 86],
            operation: Box::new(|old| old * 13),
            test: 19,
            targets: (6, 7),
            inspect_count: 0,
        }
        .into(),
        // Monkey 1:
        //   Starting items: 66, 50, 90, 53, 88, 85
        //   Operation: new = old + 3
        //   Test: divisible by 2
        //     If true: throw to monkey 5
        //     If false: throw to monkey 4
        Monkey {
            id: 1,
            items: vec![66, 50, 90, 53, 88, 85],
            operation: Box::new(|old| old + 3),
            test: 2,
            targets: (5, 4),
            inspect_count: 0,
        }
        .into(),
        // Monkey 2:
        //   Starting items: 97, 54, 89, 62, 84, 80, 63
        //   Operation: new = old + 6
        //   Test: divisible by 13
        //     If true: throw to monkey 4
        //     If false: throw to monkey 1
        Monkey {
            id: 2,
            items: vec![97, 54, 89, 62, 84, 80, 63],
            operation: Box::new(|old| old + 6),
            test: 13,
            targets: (4, 1),
            inspect_count: 0,
        }
        .into(),
        // Monkey 3:
        //   Starting items: 82, 97, 56, 92
        //   Operation: new = old + 2
        //   Test: divisible by 5
        //     If true: throw to monkey 6
        //     If false: throw to monkey 0
        Monkey {
            id: 3,
            items: vec![82, 97, 56, 92],
            operation: Box::new(|old| old + 2),
            test: 5,
            targets: (6, 0),
            inspect_count: 0,
        }
        .into(),
        // Monkey 4:
        //   Starting items: 50, 99, 67, 61, 86
        //   Operation: new = old * old
        //   Test: divisible by 7
        //     If true: throw to monkey 5
        //     If false: throw to monkey 3
        Monkey {
            id: 4,
            items: vec![50, 99, 67, 61, 86],
            operation: Box::new(|old| old * old),
            test: 7,
            targets: (5, 3),
            inspect_count: 0,
        }
        .into(),
        // Monkey 5:
        //   Starting items: 61, 66, 72, 55, 64, 53, 72, 63
        //   Operation: new = old + 4
        //   Test: divisible by 11
        //     If true: throw to monkey 3
        //     If false: throw to monkey 0
        Monkey {
            id: 5,
            items: vec![61, 66, 72, 55, 64, 53, 72, 63],
            operation: Box::new(|old| old + 4),
            test: 11,
            targets: (3, 0),
            inspect_count: 0,
        }
        .into(),
        // Monkey 6:
        //   Starting items: 59, 79, 63
        //   Operation: new = old * 7
        //   Test: divisible by 17
        //     If true: throw to monkey 2
        //     If false: throw to monkey 7
        Monkey {
            id: 6,
            items: vec![59, 79, 63],
            operation: Box::new(|old| old * 7),
            test: 17,
            targets: (2, 7),
            inspect_count: 0,
        }
        .into(),
        // Monkey 7:
        //   Starting items: 55
        //   Operation: new = old + 7
        //   Test: divisible by 3
        //     If true: throw to monkey 2
        //     If false: throw to monkey 1
        Monkey {
            id: 7,
            items: vec![55],
            operation: Box::new(|old| old + 7),
            test: 3,
            targets: (2, 1),
            inspect_count: 0,
        }
        .into(),
    ]
}
