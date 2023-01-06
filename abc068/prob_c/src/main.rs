use std::{
    collections::{HashMap, HashSet},
    vec,
};

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab_s: [(usize,usize);m]
    }

    let mut set: HashMap<usize, Vec<usize>> = HashMap::new();

    for (a, b) in ab_s {
        if a == 1 || b == n {
            if set.get_mut(&a).is_none() {
                set.insert(a, vec![b]);
            } else {
                set.get_mut(&a).unwrap().push(b);
            }
        }
    }

    let mut tmp = HashSet::new();

    set.values().for_each(|e| {
        e.iter().for_each(|f| {
            tmp.insert(f);
        })
    });

    for &i in tmp {
        let target = set.get(&i);
        if target.is_some() && target.unwrap().contains(&n) {
            println!("POSSIBLE");
            return;
        }
    }

    println!("IMPOSSIBLE");
}
