use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        _: usize,
        q: usize,
        xs: [(usize, usize, usize);q]
    }

    let mut follows = HashSet::new();

    for (t, a, b) in xs {
        match t {
            1 => {
                follows.insert((a, b));
            }
            2 => {
                follows.remove(&(a, b));
            }
            3 => {
                if follows.contains(&(a, b)) && follows.contains(&(b, a)) {
                    println!("Yes");
                } else {
                    println!("No");
                }
            }
            _ => {}
        }
    }
}
