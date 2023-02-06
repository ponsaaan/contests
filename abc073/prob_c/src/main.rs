use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a_s: [usize;n]
    }

    let mut set = HashSet::new();

    for a in a_s {
        if set.contains(&a) {
            let _ = set.remove(&a);
        } else {
            set.insert(a);
        }
    }

    println!("{}", set.len());
}
