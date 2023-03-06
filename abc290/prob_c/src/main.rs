use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut a_s: [usize;n]
    }

    a_s.sort();

    let mut set: HashSet<usize> = HashSet::new();
    for a in a_s {
        set.insert(a);
    }

    for i in 0..k {
        if !set.contains(&i) {
            println!("{}", i);
            return;
        }
    }
    println!("{}", k)
}
