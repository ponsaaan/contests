use std::vec;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut p_s:[usize;n]
    }

    let mut p_x = vec![0.0; n];
    for i in 0..n {
        p_x[i] = (p_s[i] + 1) as f64 / 2.0
    }

    let mut accum = vec![0.0; n + 1];
    for i in 0..n {
        accum[i + 1] = accum[i] + p_x[i];
    }

    let mut ans = 0.0;
    for i in 0..n - k + 1 {
        let tmp: f64 = accum[i + k] - accum[i];
        if ans < tmp {
            ans = tmp;
        }
    }

    println!("{}", ans);
}
