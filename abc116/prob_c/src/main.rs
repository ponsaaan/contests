use std::vec;

use proconio::input;

fn main() {
    input! {
        n: usize,
        h_s: [isize;n]
    }

    let mut cum = vec![0; n];
    cum[0] = h_s[0];

    for i in 1..n {
        cum[i] = h_s[i] - h_s[i - 1];
    }

    let mut ans = 0;

    for x in cum {
        if x > 0 {
            ans += x;
        }
    }

    println!("{}", ans);
}
