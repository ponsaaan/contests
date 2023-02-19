use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    if s.iter().join("") == "zyxwvutsrqponmlkjihgfedcba" {
        println!("-1");
        return;
    }

    if s.len() < 26 {
        // 使ってない文字列のうち一番先頭の文字を後にたす
        let target_string = s.clone();
        
        for c in b'a'..b'z'+1 {
            let target = c as char;
            if !target_string.contains(&target) {
                println!("{}{}", target_string.iter().join(""),target);
                return;
            }
        }
    } else {
        // 右から順番に昇順かどうかをみて行った時に、一番初めに降順になったところを起点に、
        // その文字から右側を削って、さらにその文字よりも右側にある文字の中で一番小さいものを付け足す
        for i in (0..s.len()-1).rev() {
            if s[i] < s[i+1] {
                // iを起点にそれよりも後にある文字の中で一番小さいものを見つける
                let left = s[..i].iter().collect::<String>();
                let mut targets = s[i+1..].to_vec();
                targets.sort();

                println!("{}{}", left,targets[0]);
                return;
            }
        }
    }
    println!("-1")
}
