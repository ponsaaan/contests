use std::collections::VecDeque;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut a_s: [usize;m],
    }

    let mut tmp = VecDeque::new();
    for i in 1..n+1 {
        if a_s.len() > 0 && i == a_s[0] {
            tmp.push_back(i);
            a_s.remove(0);
        } else {
            print!("{} ", i);
            while tmp.len() != 0 {
                let target = tmp.pop_back().unwrap();
                print!("{} ", target);
            }
        }
    }
}
