use std::{collections::{HashMap, BinaryHeap}, vec, cmp::{self, Reverse}};

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        abc: [(usize,usize,usize);m]
    }

    const MAX: u64 = 10000*100000+1;

    // 頂点: (接続先の頂点, 距離)
    let mut graph: HashMap<usize, Vec<(usize,u64)>> = HashMap::new();

    for (a,b,c) in abc {
        match graph.get_mut(&a) {
            Some(v) => {
                v.push((b,c as u64));
            },
            None => {
                graph.insert(a, vec![(b,c as u64)]);
                graph.insert(a, vec![(b,c as u64)]);
            },
        }

        match graph.get_mut(&b) {
            Some(v) => {
                v.push((a,c as u64));
            },
            None => {
                graph.insert(b, vec![(a,c as u64)]);
                graph.insert(b, vec![(a,c as u64)]);
            },
        }
    }

    // 各頂点が確定済みかどうか
    let mut determines = vec![false;n+1];
    // 未確定時の距離
    let mut current = vec![MAX;n+1];
    current[1] = 0;
    // 未確定要素の優先度つきキュー: (最短距離,頂点)
    let mut priority_queue = BinaryHeap::new();
    // 頂点1を入れる
    priority_queue.push(Reverse((0,1)));

    while priority_queue.len() > 0 {
        // 一番近い頂点要素を取得して確定させる
        let Reverse((dist, target)) = priority_queue.pop().unwrap();

        if determines[target] {
            continue;
        }
        determines[target] = true;

        match graph.get(&(target)) {
            // 隣接する未確定要素があれば最短距離を更新してキューに詰める
            Some(v) => {
                for &(next, cost) in v {
                    if determines[next] {
                        continue;
                    }
                    current[next] = cmp::min(current[next], dist+cost);
                    priority_queue.push(Reverse((current[next],next)));
                }
            },
            None => continue,
        }
    }

    for i in 0..n {
        if determines[i+1] {
            println!("{}", current[i+1]);
        } else {
            println!("-1");
        }
    }
}
