use std::collections::HashMap;

use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        mut s_s: [Chars;n],
    }

    // 各アルファベットの一番小さい文字数を記録する
    let mut hash_map = HashMap::new();

    for i in 0..n {
        let target = &s_s[i];
        for c in b'a'..=b'z' {
            let c_char = c as char;
            // cが何文字含まれるか
            let mut cnt = 0;
            for &cc in target {
                if cc == c_char {
                    cnt += 1;
                }
            }
            if hash_map.contains_key(&c_char) {
                // 小さい方を格納する
                if *hash_map.get(&c_char).unwrap() > cnt {
                    hash_map.insert(c_char, cnt);
                }
            } else {
                hash_map.insert(c_char, cnt);
            }
        }
    }

    let mut target_array = Vec::new();
    for (k,v) in hash_map {
        if v > 0 {
            for _ in 0..v {
                target_array.push(k);
            }
        }
    }
    target_array.sort();

    println!("{}", target_array.iter().join(""));
}