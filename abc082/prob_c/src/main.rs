use std::{collections::{HashMap}};

use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a_s: [usize;n]
    }

    let mut a_set = HashMap::new();

    for a in a_s {
        if a_set.get(&a).is_some() {
            let target = *a_set.get(&a).unwrap();
            a_set.insert(a, target+1);
        } else {
            a_set.insert(a, 1);
        }
    }

    let mut ans = 0;
    for (k,v) in a_set {
        if v >= k {
            ans += v-k
        } else {
            ans += v;
        }
    }

    println!("{}", ans)

}
