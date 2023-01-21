use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let mut ans = Vec::new();
    ans.push(s[0]);

    for i in 1..n {
        if s[i - 1] == 'n' && s[i] == 'a' {
            ans.push('y');
        }
        ans.push(s[i]);
    }

    println!("{}", ans.iter().join(""));
}
