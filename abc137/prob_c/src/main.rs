use std::{collections::HashMap, string};

use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s_s: [Chars;n]
    }

    let mut hash_map: HashMap<String, u64> = HashMap::new();

    for i in 0..n {
        let mut target = s_s[i].clone();
        target.sort();
        let target_string = target.iter().join("");
        if hash_map.contains_key(&target_string) {
            let target_num = *hash_map.get(&target_string).unwrap();
            hash_map.insert(target_string, target_num + 1);
        } else {
            hash_map.insert(target_string, 1);
        }
    }

    let mut ans: u64 = 0;
    for (_, v) in hash_map {
        ans += (v*(v-1))/2
    }
    println!("{}", ans)
}
