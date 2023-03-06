use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    }

    let mut tmp = Vec::new();

    for c in s {
        tmp.push(c.to_uppercase())
    }

    println!("{}", tmp.iter().join(""));
}
