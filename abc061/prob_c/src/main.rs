use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut K: usize,
        ab: [(usize,usize);n]
    }

    let mut target_map = BTreeMap::new();
    for (a, b) in ab {
        if target_map.get(&a).is_none() {
            target_map.insert(a, b);
        } else {
            let tmp = target_map.get(&a).unwrap();
            target_map.insert(a, tmp + b);
        }
    }

    for (k, v) in target_map {
        if v >= K {
            println!("{}", k);
            return;
        }
        K -= v;
    }
}
