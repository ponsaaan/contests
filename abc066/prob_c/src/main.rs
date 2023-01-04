use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a_s: [usize;n]
    }

    let mut ans = VecDeque::new();

    if n % 2 == 0 {
        // 右左の順にpush
        for i in 0..n {
            if i % 2 == 0 {
                ans.push_back(a_s[i]);
            } else {
                ans.push_front(a_s[i]);
            }
        }
    } else {
        // 左右の順にpush
        for i in 0..n {
            if i % 2 == 0 {
                ans.push_front(a_s[i]);
            } else {
                ans.push_back(a_s[i]);
            }
        }
    }

    for i in ans {
        print!("{} ", i);
    }
    println!()
}
