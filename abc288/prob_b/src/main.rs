use proconio::{input};

fn main() {
    input! {
        n: usize,
        k: usize,
        mut s_s: [String;n]
    }

    let mut tmp = Vec::new();
    for i in 0..k {
        tmp.push(&s_s[i]);
    }

    tmp.sort();

    for s in tmp {
        println!("{}", s)
    }
}
