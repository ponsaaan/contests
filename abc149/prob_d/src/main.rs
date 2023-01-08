use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        k: usize,
        r: usize,
        s: usize,
        p: usize,
        mut t: Chars,
    }

    let mut ans = 0;
    for i in 0..n {
        if i >= k && t[i] == t[i - k] {
            t[i] = 'x';
        }
        match t[i] {
            'r' => ans += p,
            's' => ans += r,
            'p' => ans += s,
            _ => {}
        }
    }

    println!("{}", ans);
}
