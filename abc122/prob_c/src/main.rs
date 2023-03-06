use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        q: usize,
        s: Chars,
        lr: [(usize,usize);q]
    }

    // ACがどのindexにあるか
    let mut tmp = vec![0usize;n];
    let mut tmp_cnt = 0;
    for i in 0..n-1 {
        if s[i] == 'A' && s[i+1] == 'C' {
            tmp_cnt += 1;
        }
        tmp[i] = tmp_cnt;
    }

    for (l, r) in lr {
        if l == 1 {
            println!("{}", tmp[r-2]);
        
        } else {
            println!("{}", tmp[r-2]-tmp[l-2])
        }
    }
}
