use std::collections::{VecDeque, HashMap};

use proconio::input;

fn main() {
    input! {
        t: usize, // t <= 1000
    }

    for _ in 0..t {
        input! {
            n: usize, // 2 <= n <= 2000
            m: usize,
            c_s: [usize;n], // 0 or 1
            uv: [(usize,usize);m] // 1 ~ n
        }

        // グラフの作成
        let mut hash_map: HashMap<usize, Vec<usize>> = HashMap::new();
        for (u,v) in uv {
            if hash_map.contains_key(&u) {
                let target = hash_map.get(&u).unwrap();
                let mut tmp_array = Vec::new();
                for &t in target {
                    tmp_array.push(t);
                }
                tmp_array.push(v);
                hash_map.insert(u, tmp_array);
            } else {
                hash_map.insert(u, vec![v]);
            }
        }

        let mut already = vec![-1;n+1];
        let mut queue = VecDeque::new();
        queue.push_back(1);
        already[1] = 0;
        while queue.len() != 0 {
            let target  = queue.pop_front().unwrap();
            if hash_map.get(&target).is_some() {
                let hoge = hash_map.get(&target).unwrap();
                for &h in hoge {
                    if already[h] == -1 && c_s[target-1] != c_s[h-1] {
                        already[h] = already[target]+1;
                        queue.push_back(h);
                    }
                }
            }
        }
        
        if already[n] != -1 {
            println!("{}", already[n])
        } else {
            println!("-1")
        }
    }
}
