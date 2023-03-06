use std::collections::{HashMap};

use petgraph::{unionfind::UnionFind};
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        uv: [(usize,usize);m]
    }
    // ROOT番号：（頂点数,辺数）
    let mut hash_map: HashMap<usize, (usize,usize)> = HashMap::new();
    (1..n+1).for_each(|item| {hash_map.insert(item,(1,0));});

    let mut g:UnionFind<usize> = UnionFind::new(n+1);
    for i in 0..m {
        let (u,v) = uv[i];
        if g.equiv(u, v) {
            // 同じグループなら辺を+1
            let pre = g.find(u);
            g.union(u, v);
            let post = g.find(u);
            if pre != post {
                hash_map.insert(post, *hash_map.get(&pre).unwrap());
                hash_map.get_mut(&post).unwrap().1 += 1;
            } else {
                hash_map.get_mut(&post).unwrap().1 += 1;
            }
            
        } else {
            // 別成分なら頂点と辺を+1
            g.union(u, v);
            let target = g.find(u);
            hash_map.get_mut(&target).unwrap().0 += 1;
            hash_map.get_mut(&target).unwrap().1 += 1;
        }
    }

    for i in 1..n+1 {
        let target = hash_map.get(&g.find(i)).unwrap();
        if target.0 != target.1 {
            println!("No");
            return;
        }
    }
    println!("Yes");
}
