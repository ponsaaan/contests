use std::cmp;
use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let mut ans = 0;

    for i in 0..n {
        let mut left_unique = HashSet::new();
        let mut right_unique = HashSet::new();

        for j in 0..i + 1 {
            left_unique.insert(s[j]);
        }
        for j in i + 1..n {
            right_unique.insert(s[j]);
        }

        let mut tmp = 0;

        for k in &left_unique {
            for l in &right_unique {
                if *k == *l {
                    tmp += 1
                }
            }
        }

        ans = cmp::max(ans, tmp);
    }

    println!("{}", ans);
}
