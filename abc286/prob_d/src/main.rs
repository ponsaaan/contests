use std::collections::{hash_set, HashSet};

use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        ab: [(usize,usize);n]
    }

    let mut hash_set = HashSet::new();
    let first = ab[0];
    for i in 0..ab[0].1 + 1 {
        hash_set.insert(first.0 * i);
    }

    for (i, (a, b)) in ab.iter().enumerate() {
        if i == 0 {
            continue;
        }

        for k in hash_set.clone() {
            for l in 0..b + 1 {
                hash_set.insert(k + a * l);
            }
        }
    }

    if hash_set.contains(&x) {
        println!("Yes")
    } else {
        println!("No")
    }
}
