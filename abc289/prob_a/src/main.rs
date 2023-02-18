use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    }

    let mut hoge = Vec::new();
    for c in s {
        if c == '0' {
            hoge.push('1')
        } else {
            hoge.push('0')
        }
    }
    println!("{}", hoge.iter().join(""));
}
