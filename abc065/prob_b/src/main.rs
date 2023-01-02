use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a_s: [usize;n]
    }

    let mut target_map = HashMap::new();
    for (i, a) in a_s.iter().enumerate() {
        target_map.insert(i + 1, *a);
    }

    let mut next_button: usize = 1;

    for i in 0..n {
        if next_button == 2 {
            println!("{}", i);
            return;
        }
        next_button = *target_map.get(&next_button).unwrap();
    }
    println!("{}", -1);
}
