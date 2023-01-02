use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars,
    }

    for _ in 0..(s.len() / 2) {
        s.pop();
        s.pop();

        let half_index = s.len() / 2;
        if s[0..half_index] == s[half_index..] {
            println!("{}", s.len());
            return;
        }
    }
}
