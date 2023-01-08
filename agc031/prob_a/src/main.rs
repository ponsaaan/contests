use std::collections::HashMap;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        _: usize,
        c_s: Chars,
    }

    let mut map: HashMap<char, u64> = HashMap::new();

    for c in c_s {
        if map.get(&c).is_none() {
            map.insert(c, 1);
        } else {
            let tmp = *map.get(&c).unwrap();
            map.insert(c, tmp + 1);
        }
    }

    let mut ans = 1u64;
    for (_, i) in map {
        ans *= i + 1;
        ans %= 1000000007;
    }

    println!("{}", (ans - 1))
}
