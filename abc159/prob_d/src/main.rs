use std::{vec};

use proconio::input;

fn main() {
    input! {
        n: usize,
        a_s: [usize;n]
    }

    // カウント用
    let mut tmp: Vec<u64> = vec![0;200001];
    for &a in &a_s {
        tmp[a] += 1;
    }

    let mut sum = 0;
    for &i in &tmp {
        if i>1 {
            sum += i * (i-1) / 2;
        }
    }

    for &a in &a_s {
        if tmp[a] == 1 {
            println!("{}", sum);
        } else if tmp[a] == 2 {
            println!("{}", sum-1);
        } else {
            println!("{}", sum-(tmp[a]-1));
        }
    }
}
