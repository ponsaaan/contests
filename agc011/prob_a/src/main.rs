use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        n: usize,
        c: usize,
        k: usize,
        mut t_s: [usize;n],
    }
    // c:乗客数
    // k:待ち時間上限
    t_s.sort();

    let mut ans = 0;
    let mut passenger_cnt = 0;
    let mut criteria = t_s[0];
    for i in 0..n {
        // 満員になるか制限時間がくれば次のバス
        if passenger_cnt == c || t_s[i] > criteria + k {
            passenger_cnt = 0;
            criteria = 0;
            ans += 1;
        }
        if criteria == 0 {
            criteria = t_s[i];
        }
        passenger_cnt += 1;
    }
    if passenger_cnt > 0 {
        ans += 1;
    }
    println!("{}", ans);
}
