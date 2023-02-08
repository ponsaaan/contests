use std::collections::VecDeque;

use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    }

    let mut stack = VecDeque::new();
    for c in s {
        if c == '0' || c == '1' {
            stack.push_back(c)
        } else {
            let _ = stack.pop_back();
        }
    }

    println!("{}", stack.iter().join(""));

}
