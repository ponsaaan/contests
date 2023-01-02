use std::vec;

use proconio::{input, marker::Chars};
use std::cmp;

fn main() {
    input! {
        s: Chars,
    }

    let mut a = vec![0; s.len() + 1];

    for i in 0..s.len() {
        if s[i] == '<' {
            a[i + 1] = cmp::max(a[i + 1], a[i] + 1)
        }
    }

    for i in (0..s.len()).rev() {
        if s[i] == '>' {
            a[i] = cmp::max(a[i], a[i + 1] + 1)
        }
    }

    let sum: usize = a.iter().sum();
    println!("{}", sum)
}
