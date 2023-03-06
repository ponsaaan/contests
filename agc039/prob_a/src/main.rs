use proconio::{input, marker::Chars};
use std::{thread, time};

fn main() {
    input! {
        s: Chars,
        k: u64,
    }

    // 何回sの文字列を入れ替える操作をいおこなれば良いか
    let mut ans = 0;
    // 同じ文字が連続して何回現れるか
    let mut tmp_cnt = 1;

    for i in 1..s.len() {
        if s[i] == s[i-1] {
            tmp_cnt += 1;
        } else {
            ans += tmp_cnt / 2;
            tmp_cnt = 1;
        }
    }
    ans += tmp_cnt / 2;

    let duration = time::Duration::from_millis(2001);

    // 全部同じ文字
    let first = s[0];
    if s.iter().all(|x| *x == first) {
        println!("{}", (s.len() as u64 *k)/2);
        return;
    }

    // 両サイドが別の文字
    if s[0] != s[s.len()-1] {
        println!("{}", ans * k);
        return;
    }

    // 左側で続く文字数
    

    if s[0] != s[1] && s[s.len()-1] != s[s.len()-2] {
        // 両サイドに同じ文字が1つずつある
        println!("{}", ans * k + k - 1);
    } else {
        println!("{}", ans * k);
    }
}
