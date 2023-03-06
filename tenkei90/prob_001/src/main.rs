use std::cmp;

use proconio::input;

fn main() {
    input! {
        n: usize,
        l: usize,
        k: usize,
        a_s: [usize;n]
    }

    let mut tmp = Vec::new();

    for i in 0..n {
        if i == 0 {
            tmp.push(a_s[i]);
        } else {
            tmp.push(a_s[i]-a_s[i-1]);
        }
    }
    tmp.push(l - a_s[n-1]);

    tmp.sort_by_key(|&x| cmp::Reverse(x));

    println!("{:?}", tmp);

    println!("{}", tmp[k+1]);
}
