use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a_s:[usize;n],
        m: usize,
        b_s: [usize;m],
        x: usize,
    }

    let mut dp = vec![false;x+1];
    let mut hash_set = HashSet::new();
    for i in 0..m {
        hash_set.insert(b_s[i]);
    }
    for i in 1..x+1 {
        if hash_set.contains(&i) {
            continue;
        }
        if a_s.contains(&i) {
            dp[i] = true;
            continue;
        }
        for j in 0..n {
            if i >= a_s[j] && i <= x + a_s[j] {
                if dp[i - a_s[j]] {
                    dp[i] = true;
                }
            }
        }
    }

    if dp[x] {
        println!("Yes")
    } else {
        println!("No")
    }

}
