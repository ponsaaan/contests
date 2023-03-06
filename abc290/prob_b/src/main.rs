use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        k: usize,
        s: Chars,
    }

    let mut remain_count = k;
    let mut ans = Vec::new();
    for i in 0..n {
        if remain_count == 0 {
            ans.push('x');
            continue;
        }
        if s[i] == 'o' {
            ans.push('o');
            remain_count -= 1
        } else {
            ans.push('x');
        }
    }

    println!("{}", ans.iter().join(""))
}
