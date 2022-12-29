const INPUT: &str = include_str!("input");

type Element = (isize, isize);
type ElementVec = Vec<Element>;

fn math_mod(mut value: isize, modulo: isize) -> isize {
    if value >= modulo {
        value % modulo
    } else if value < 0 {
        let factor = ((-value) / modulo) + 1;

        value += factor * modulo;
        value % modulo
    } else {
        value
    }
}

fn move_element(list: &mut ElementVec, x: usize) {
    let (idx, delta) = list[x];

    if delta == 0 {
        return;
    }

    let mut new_idx = math_mod(idx + delta, (list.len() - 1) as isize);

    if new_idx == 0 {
        new_idx = (list.len() - 1) as isize;
    }

    list[x].0 = new_idx;

    let range_start = idx.min(new_idx);
    let range_end = idx.max(new_idx);
    let range = range_start..=range_end;
    let move_delta = if idx < new_idx { -1 } else { 1 };
    for (list_index, (move_index, _delta)) in list.iter_mut().enumerate() {
        if list_index == x {
            continue;
        }
        if range.contains(move_index) {
            *move_index += move_delta;
        }
    }
}

fn create_list(input: &str) -> ElementVec {
    input
        .split(&['\n', ','])
        .enumerate()
        .map(|(idx, v)| (idx as isize, v.trim().parse::<isize>().unwrap()))
        .collect()
}

fn modify_list(list: &mut ElementVec, key: isize) {
    list.iter_mut().for_each(|e| e.1 *= key);
}

fn get_result_list(mut list: ElementVec) -> Vec<isize> {
    list.sort_by(|a, b| a.0.cmp(&b.0));

    list.iter().map(|e| e.1).collect()
}

fn get_coordinates(coordinates: Vec<isize>) -> isize {
    let idx0 = coordinates.iter().position(|i| i == &0).unwrap();

    let idx1000 = (idx0 + 1000) % coordinates.len();
    let idx2000 = (idx0 + 2000) % coordinates.len();
    let idx3000 = (idx0 + 3000) % coordinates.len();

    let a = coordinates[idx1000];
    let b = coordinates[idx2000];
    let c = coordinates[idx3000];

    a + b + c
}

fn do_mix(mut list: &mut ElementVec) {
    for x in 0..list.len() {
        move_element(&mut list, x);
    }
}

fn main() {
    let mut list = create_list(INPUT);

    do_mix(&mut list);

    let coordinates = get_coordinates(get_result_list(list));

    println!("Part 1: {coordinates}");

    let mut list = create_list(INPUT);

    modify_list(&mut list, 811589153);
    for _ in 0..10 {
        do_mix(&mut list);
    }

    let coordinates = get_coordinates(get_result_list(list));

    println!("Part 2: {coordinates}");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_math_mod() {
        assert_eq!(math_mod(-9000, 5000), 1000);
        assert_eq!(math_mod(0, 5000), 0);
        assert_eq!(math_mod(500, 5000), 500);
        assert_eq!(math_mod(7000, 5000), 2000);
        assert_eq!(math_mod(5000, 5000), 0);
    }

    #[test]
    fn test_move() {
        let mut list = create_list("4, 5, 6, 1, 7, 8, 9");
        move_element(&mut list, 3);
        assert_eq!(get_result_list(list), vec![4, 5, 6, 7, 1, 8, 9])
    }

    #[test]
    fn test_move_negative() {
        let mut list = create_list("4, -2, 5, 6, 7, 8, 9");
        move_element(&mut list, 1);
        assert_eq!(get_result_list(list), vec![4, 5, 6, 7, 8, -2, 9])
    }

    #[test]
    fn test_function() {
        const TEST: &str = include_str!("test");

        let mut list = create_list(TEST);

        do_mix(&mut list);

        assert_eq!(get_result_list(list), vec![1, 2, -3, 4, 0, 3, -2]);
    }
    #[test]
    fn test_failing_step_in_testfile() {
        let mut list = create_list("1, 2, -2, -3, 0, 3, 4");

        move_element(&mut list, 2);

        assert_eq!(get_result_list(list.clone()), vec![1, 2, -3, 0, 3, 4, -2]);
    }

    #[test]
    fn coordinates_calculated_correctly() {
        const TEST: &str = include_str!("test");

        let mut list = create_list(TEST);

        do_mix(&mut list);

        let coordinate = get_coordinates(get_result_list(list));

        assert_eq!(coordinate, 3);
    }

    #[test]
    fn modify_list_works() {
        const TEST: &str = include_str!("test");
        let mut list = create_list(TEST);

        modify_list(&mut list, 811589153);

        assert_eq!(
            get_result_list(list),
            vec![
                811589153,
                1623178306,
                -2434767459,
                2434767459,
                -1623178306,
                0,
                3246356612
            ]
        );
    }
    #[test]
    fn test_mixing_rounds() {
        const TEST: &str = include_str!("test");
        let mut list = create_list(TEST);

        modify_list(&mut list, 811589153);

        do_mix(&mut list);
        assert_eq!(
            get_result_list(list.clone()),
            vec![
                0,
                -2434767459,
                3246356612,
                -1623178306,
                2434767459,
                1623178306,
                811589153
            ]
        );
        do_mix(&mut list);
        assert_eq!(
            get_result_list(list.clone()),
            vec![
                0,
                2434767459,
                1623178306,
                3246356612,
                -2434767459,
                -1623178306,
                811589153
            ]
        );
        do_mix(&mut list);
        assert_eq!(
            get_result_list(list.clone()),
            vec![
                0,
                811589153,
                2434767459,
                3246356612,
                1623178306,
                -1623178306,
                -2434767459
            ]
        );
        do_mix(&mut list);
        assert_eq!(
            get_result_list(list.clone()),
            vec![
                0,
                1623178306,
                -2434767459,
                811589153,
                2434767459,
                3246356612,
                -1623178306
            ]
        );
        do_mix(&mut list);
        assert_eq!(
            get_result_list(list.clone()),
            vec![
                0,
                811589153,
                -1623178306,
                1623178306,
                -2434767459,
                3246356612,
                2434767459
            ]
        );
        do_mix(&mut list);
        assert_eq!(
            get_result_list(list.clone()),
            vec![
                0,
                811589153,
                -1623178306,
                3246356612,
                -2434767459,
                1623178306,
                2434767459
            ]
        );
        do_mix(&mut list);
        assert_eq!(
            get_result_list(list.clone()),
            vec![
                0,
                -2434767459,
                2434767459,
                1623178306,
                -1623178306,
                811589153,
                3246356612
            ]
        );
        do_mix(&mut list);
        assert_eq!(
            get_result_list(list.clone()),
            vec![
                0,
                1623178306,
                3246356612,
                811589153,
                -2434767459,
                2434767459,
                -1623178306
            ]
        );
        do_mix(&mut list);
        assert_eq!(
            get_result_list(list.clone()),
            vec![
                0,
                811589153,
                1623178306,
                -2434767459,
                3246356612,
                2434767459,
                -1623178306
            ]
        );
        do_mix(&mut list);
        assert_eq!(
            get_result_list(list.clone()),
            vec![
                0,
                -2434767459,
                1623178306,
                3246356612,
                -1623178306,
                2434767459,
                811589153
            ]
        );
    }

    #[test]
    fn test_part2_coordinate() {
        const TEST: &str = include_str!("test");
        let mut list = create_list(TEST);

        modify_list(&mut list, 811589153);

        for _ in 0..10 {
            do_mix(&mut list);
        }

        let coordinate = get_coordinates(get_result_list(list));

        assert_eq!(coordinate, 1623178306);
    }
}
