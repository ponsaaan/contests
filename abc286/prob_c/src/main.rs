use std::cmp;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        a: u64,
        b: u64,
        s: Chars,
    }

    let len = s.len();

    let mut ans = a * 5000 + b * 5000;
    for i in 0..n {
        let mut s_copy = s.clone();

        for _ in 0..i {
            let first = s_copy[0];
            s_copy.remove(0);
            s_copy.push(first);
        }

        let md = len / 2;
        let mut b_cnt = 0;
        if len % 2 == 0 {
            for (cnt, j) in (md..len).enumerate() {
                if s_copy[j] != s_copy[j - (2 * (cnt + 1) - 1)] {
                    b_cnt += 1;
                }
            }
        } else {
            for (cnt, j) in (md + 1..len).enumerate() {
                if s_copy[j] != s_copy[j - (2 * (cnt + 1))] {
                    b_cnt += 1;
                }
            }
        }
        ans = cmp::min(ans, a * (i as u64) + b * b_cnt);
    }
    println!("{}", ans);
}
