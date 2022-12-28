use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        s: [Chars;h],
        t: [Chars;h],
    }

    let mut new_s = vec![];
    let mut new_t = vec![];

    for j in 0..w {
        // 列ごとに走査
        new_s.push((0..h).map(|i| s[i][j]).join(""));
        new_t.push((0..h).map(|i| t[i][j]).join(""));
    }

    new_s.sort();
    new_t.sort();

    if new_s == new_t {
        println!("Yes");
    } else {
        println!("No");
    }
}
