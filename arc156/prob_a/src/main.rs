use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        t: usize,
    }

    for _ in 0..t {
        input! {
            n: usize,
            mut s: Chars
        }

        let mut cnt = 0;

        for j in 0..n {
            if s[j] == '1' {
                cnt += 1;
            }
        }

        if cnt % 2 == 0 {
            if cnt == 2 {
                if s.len() == 3 {
                    if s[0] == '1' && s[1] == '0' && s[2] == '1'  {
                        println!("1")
                    } else {
                        println!("-1")
                    }
                } else if s.len() == 4 {
                    if s.len()==4 && s[0] == '0' && s[1] == '1' && s[2] == '1' && s[3] == '0' {
                        println!("3");
                        continue;
                    }
                    // 連続する11があれば必ず操作回数は2になる
                    if s.iter().join("").contains("11") {
                        println!("{}", 2);
                        continue;
                    }
                    println!("{}", cnt/2)
                }else {
                    // 連続する11があれば必ず操作回数は2になる
                    if s.iter().join("").contains("11") {
                        println!("{}", 2);
                        continue;
                    }
                    println!("{}", cnt/2)
                }
                continue;
            }
            println!("{}", cnt/2)
        } else {
            println!("-1")
        }
    }
}
