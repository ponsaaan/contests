use std::cmp;

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a_s: [usize;n]
    }

    a_s.sort();

    let mut count = vec![0; 100000];
    for &a in &a_s {
        count[a] += 1;
    }

    let mut ans = 0;

    for x in 0..99998 {
        ans = cmp::max(ans, count[x] + count[x + 1] + count[x + 2]);
    }

    println!("{}", ans);
}
