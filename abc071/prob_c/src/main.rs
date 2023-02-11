use std::cmp;

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a_s: [isize;n]
    }

    a_s.sort_by_key(|&x| cmp::Reverse(x));

    let mut tmp = Vec::new();
    let mut pre = -1;
    let mut is_ok = true;
    for i in 0..n {
        if is_ok && pre == a_s[i] {
            tmp.push(a_s[i]);
            is_ok = false;
        } else {
            is_ok = true;
        }
        pre = a_s[i]
    }

    if tmp.len() < 2 {
        println!("0")
    } else {
        println!("{}", tmp[0]*tmp[1])
    }
}
