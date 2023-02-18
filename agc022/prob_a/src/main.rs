use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    if s.len() < 26 {
        // 使ってない文字列のうち一番先頭の文字を後にたす
        let target_string = s.clone();
        
        for c in b'a'..b'z'+1 {
            let target = c as char;
            if !target_string.contains(&target) {
                println!("{}{}", target_string.iter().join(""),c);
                return;
            }
        }
    } else {
        // 右から順番に昇順かどうかをみて行った時に、一番初めに降順になったところを起点に、
        // その文字を削って、さらにその文字よりも右側にある文字の中で一番小さいものを付け足す
        let s_clone = s.clone().iter().rev().collect();
        for i in 1..s.len() {
            if s_clone[i] < s_clone[i-1] {
                // iを起点にそれよりも前にある文字の中で一番小さいものを見つける
                let target = s_clone[..i-1];
            }
        }


        for i in 0..s.len()-1 {
            let target_i = s.len() - i - 1;
            if s[target_i-1] < s[target_i] {
                
                return;
            }
        }
    }
}
