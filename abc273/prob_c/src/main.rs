use itertools::Itertools;
use proconio::input;
use superslice::Ext;

fn main() {
    input! {
        n: usize,
        an: [usize;n]
    }

    let mut b = an.clone();
    b.sort_unstable();
    // 連続する重複を削除
    b.dedup();

    let mut result = vec![0; n];
    for x in an {
        result[b.len() - b.upper_bound(&x)] += 1;
    }

    println!("{}", result.iter().join("\n"));
}
