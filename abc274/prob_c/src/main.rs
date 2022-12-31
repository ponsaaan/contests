use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        x: [usize;n],
    }

    let mut result: BTreeMap<usize, usize> = BTreeMap::new();
    result.insert(1, 0);

    for (i, a) in x.iter().enumerate() {
        let target = *result.get(a).unwrap();
        result.insert(2 * (i + 1), target + 1);
        result.insert(2 * (i + 1) + 1, target + 1);
    }

    for i in result.values() {
        println!("{}", i);
    }
}
