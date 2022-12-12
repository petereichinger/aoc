use std::cell::RefCell;

use crate::monkey::Monkey;

pub fn get_monkeys() -> Vec<RefCell<Monkey>> {
    vec![
        // Monkey 0:
        //   Starting items: 79, 98
        //   Operation: new = old * 19
        //   Test: divisible by 23
        //     If true: throw to monkey 2
        //     If false: throw to monkey 3
        Monkey {
            id: 0,
            items: vec![79, 98],
            operation: Box::new(|old| old * 19),
            test: 23,
            targets: (2, 3),
            inspect_count: 0,
        }
        .into(),
        // Monkey 1:
        //   Starting items: 54, 65, 75, 74
        //   Operation: new = old + 6
        //   Test: divisible by 19
        //     If true: throw to monkey 2
        //     If false: throw to monkey 0
        Monkey {
            id: 1,
            items: vec![54, 65, 75, 74],
            operation: Box::new(|old| old + 6),
            test: 19,
            targets: (2, 0),
            inspect_count: 0,
        }
        .into(),
        // Monkey 2:
        //   Starting items: 79, 60, 97
        //   Operation: new = old * old
        //   Test: divisible by 13
        //     If true: throw to monkey 1
        //     If false: throw to monkey 3
        Monkey {
            id: 2,
            items: vec![79, 60, 97],
            operation: Box::new(|old| old * old),
            test: 13,
            targets: (1, 3),
            inspect_count: 0,
        }
        .into(),
        // Monkey 3:
        //   Starting items: 74
        //   Operation: new = old + 3
        //   Test: divisible by 17
        //     If true: throw to monkey 0
        //     If false: throw to monkey 1
        Monkey {
            id: 3,
            items: vec![74],
            operation: Box::new(|old| old + 3),
            test: 17,
            targets: (0, 1),
            inspect_count: 0,
        }
        .into(),
    ]
}
