use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let length = s.len();

    // 取り除く文字数
    let target_i = length - 7;

    for i in 0..length {
        if i+target_i >= length {
            break;
        }
        let mut tmp = s.clone();
        tmp.drain(i..(i+target_i));

        if tmp.iter().join("") == "keyence" {
            println!("YES");
            return;
        }
    }

    println!("NO");
}
