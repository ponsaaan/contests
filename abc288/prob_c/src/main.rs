use petgraph::unionfind::UnionFind;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize,usize);m]
    }

    let mut ans = 0;
    let mut union_find:UnionFind<usize>= UnionFind::new(n);
    for (a,b) in ab {
        if union_find.union(a-1, b-1) {
            // 何もしない
        } else {
            ans += 1
        }
    }

    println!("{}", ans);

}
