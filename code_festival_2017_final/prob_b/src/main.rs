use num::Signed;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let mut cnt_a = 0;
    let mut cnt_b = 0;
    let mut cnt_c = 0;

    for c in s {
        if c == 'a' {
            cnt_a += 1;
        } else if c == 'b' {
            cnt_b += 1;
        } else {
            cnt_c += 1;
        }
    }

    if (cnt_a-cnt_b).abs() > 1 || 
    (cnt_b-cnt_c).abs() > 1 || 
    (cnt_a-cnt_c).abs() > 1 {
        println!("NO")
    } else {
        println!("YES")
    }
}
