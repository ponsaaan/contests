use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        s_s: [Chars;h]
    }

    for i in 0..h {
        for j in 0..w {
            if s_s[i][j] == '#' {
                // 上下左右にが全て.だったらout
                if j>=1 && s_s[i][j-1] == '#' {
                    continue;
                }
                if j+1<w && s_s[i][j+1] == '#' {
                    continue;
                }
                if i>=1 && s_s[i-1][j] == '#' {
                    continue;
                }
                if i+1<h && s_s[i+1][j] == '#' {
                    continue;
                }
                println!("No");
                return;
            }
        }
    }
    println!("Yes");


}
