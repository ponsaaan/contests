use std::collections::VecDeque;

use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
        q: usize,
    }

    let mut is_direction_right = true;

    let mut ans: VecDeque<char> = VecDeque::new();
    for c in s {
        ans.push_back(c);
    }
    for _ in 0..q {
        input! {
            t: usize,
        }

        if t == 1 {
            // 反転
            is_direction_right = !is_direction_right;
        } else {
            // 追加
            input! {
                f: usize,
                c: char,
            }
            if f == 1 {
                //左から追加
                if is_direction_right {
                    ans.push_front(c);
                } else {
                    ans.push_back(c);
                }
            } else {
                //右から追加
                if is_direction_right {
                    ans.push_back(c);
                } else {
                    ans.push_front(c);
                }
            }
        }
    }
    if is_direction_right {
        println!("{}", ans.iter().join(""));
    } else {
        println!("{}", ans.iter().rev().join(""));
    }
}
