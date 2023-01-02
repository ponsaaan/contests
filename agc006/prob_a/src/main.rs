use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
        t: Chars,
    }

    for i in 0..n {
        if s[i..] == t[0..n - i] {
            println!("{}", n + i);
            return;
        }
    }

    println!("{}", 2 * n);
}
