use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    }

    let mut ans = 0;
    for i in 1..s.len() {
        if s[i] != s[i - 1] {
            ans += 1;
        }
    }

    println!("{}", ans);
}
