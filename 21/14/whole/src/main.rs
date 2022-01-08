use std::collections::HashMap;
use itertools::Itertools;
use utils::read_input_by_lines;


fn main() {
    let mut lines = read_input_by_lines();

    let initial = lines.next().unwrap();

    let first = initial.chars().nth(0).unwrap();

    lines.next();

    let mappings: HashMap<_, _> = lines.map(|l| {
        let (old, new) = l.split_once(" -> ").unwrap();
        (old.to_owned(), new.to_owned())
    }).collect();

    let mut bonds = HashMap::new();

    for bond in initial.chars().tuple_windows().map(|(f, s)| String::from_iter([f, s].iter())) {
        *bonds.entry(bond).or_insert(0) += 1u128;
    }

    let mut new_bonds = HashMap::new();

    for _iter in 0..40 {
        new_bonds.clear();

        for (bond, count) in &bonds {
            let (f, s) = bond.split_at(1);
            let new = mappings.get(bond).unwrap();

            let f = f.to_owned() + new;
            let s = new.clone() + s;

            *new_bonds.entry(f).or_insert(0) += count;
            *new_bonds.entry(s).or_insert(0) += count;
        }

        std::mem::swap(&mut bonds, &mut new_bonds);
    }

    let mut atom_counts = HashMap::new();

    for (bond, count) in &bonds {
        *atom_counts.entry(bond.chars().nth(1).unwrap()).or_insert(0) += count;
    }

    *atom_counts.entry(first).or_insert(0) += 1;

    let ((_, min), (_, max)) = atom_counts.iter().minmax_by(|(a, acnt), (b, bcnt)| acnt.cmp(bcnt)).into_option().unwrap();

    println!("{} {} {}", max, min, max - min);

    // dbg!(atom_counts);
}