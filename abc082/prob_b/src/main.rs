use std::cmp::Reverse;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars,
        mut t: Chars,
    }

    s.sort();
    t.sort_by_key(|&x| Reverse(x));

    if s < t {
        println!("Yes")
    } else {
        println!("No")
    }
 }
