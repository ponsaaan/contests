use std::collections::{HashMap, HashSet};

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        sc: [(usize,usize);m]
    }

    if n == 1 {
        if m > 0 {
            let mut already = HashSet::new();
            for (_,c) in &sc {
                if already.len() > 0 {
                    if !already.contains(&c) {
                        println!("-1");
                    return;
                    }
                } else {
                    already.insert(c);
                }
            }
            println!("{}", sc[0].1);
        } else {
            println!("0");
        }
    } else if n==2 {
        let mut vec = vec![0;2];
        let mut already_map = HashMap::new();
            for (s,c) in sc {
                if s == 1 && c == 0 {
                    println!("-1");
                    return;
                }
                if already_map.contains_key(&s) {
                    if *already_map.get(&s).unwrap() != c {
                        println!("-1");
                        return;
                    }
                }
                already_map.insert(s, c);
                vec[s-1] = c;
            }
            if vec[0] == 0 {
                vec[0] = 1;
            }
            println!("{}", vec.iter().join(""));
    } else {
        let mut vec = vec![0;3];
        let mut already_map = HashMap::new();
            for (s,c) in sc {
                if s == 1 && c == 0 {
                    println!("-1");
                    return;
                }
                if already_map.contains_key(&s) {
                    if *already_map.get(&s).unwrap() != c {
                        println!("-1");
                        return;
                    }
                }
                already_map.insert(s, c);
                vec[s-1] = c;
            }
            if vec[0] == 0 {
                vec[0] = 1;
            }
            println!("{}", vec.iter().join(""));
    }
}
