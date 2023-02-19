use core::unicode::conversions;
use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        mut a_s: [usize;n]
    }
    let mut hash_set_first = HashSet::new();
    let mut hash_set = HashSet::new();
    
    for a in a_s {
        hash_set_first.insert(a);
    }
    let mut ans_k = vec![0;k];

    let cnt = 0;
    for i in 0..k {
        hash_set.insert(i);
        // 連続する数字がなくなるまでループする
        for j in 0..200000 {
            if !hash_set.contains(&i) {
                hash_set.insert(i);
                cnt = hash_set.len();
                break;
            } else {
                continue;
            }
        }
    }
    // TODO: ans = 0のバリデーション

    let mut ans = 1;
    for i in 0..k {
        ans *= ans_k[i] % 998244353;
    }

    println!("{}", ans % 998244353);

}
