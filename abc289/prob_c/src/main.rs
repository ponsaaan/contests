use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    let mut sets = Vec::new();

    for i in 0..m {
        input! {
            c: usize,
            c_s: [usize;c]
        }
        sets.push(c_s);
    }

    let mut ans = 0;

    for i in 0..1<<m {
        let mut tmp_set = HashSet::new();
        for j in 0..m {
            if i & (1<<j) != 0 {
                for &c in &sets[j] {
                    tmp_set.insert(c);
                }
            }
        }
        let mut tmp_yes = true;
        for hoge in 1..n+1 {
            if !tmp_set.contains(&hoge) {
                tmp_yes = false;
                break;
            }
        }
        if tmp_yes {
            ans += 1;
        }
    }
    println!("{}", ans)
}
