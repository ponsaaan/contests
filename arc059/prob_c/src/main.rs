use std::cmp;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a_s: [isize;n]
    }

    let min = *a_s.iter().min().unwrap();
    let max = *a_s.iter().max().unwrap();

    let mut ans = 200 * 200 * n;
    for y in min..=max {
        let mut tmp = 0;
        for &x in &a_s {
            tmp += (x - y) * (x - y);
        }
        ans = cmp::min(ans, tmp as usize);
    }

    println!("{}", ans);
}
