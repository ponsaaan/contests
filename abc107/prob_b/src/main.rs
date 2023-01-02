use std::vec;

use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        mut a_s: [Chars;h]
    }

    let mut skip_h = vec![false; h]; // 行スキップ
    let mut skip_w = vec![false; w]; // 列スキップ

    for i in 0..h {
        skip_h[i] = a_s[i].iter().all(|&x| x == '.');
    }
    for j in 0..w {
        skip_w[j] = (0..h).all(|i| a_s[i][j] == '.');
    }

    let mut ans = vec![];

    for i in 0..h {
        if skip_h[i] {
            continue;
        }
        for j in 0..w {
            if skip_w[j] {
                continue;
            }
            ans.push(a_s[i][j])
        }
        ans.push('\n');
    }

    println!("{}", ans.iter().join(""));
}
