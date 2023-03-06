use proconio::{input, marker::Chars};


fn main() {
    input! {
        h: usize,
        w: usize,
        a_s: [Chars;h]
    }

    let mut count = 0;
    for i in 0..h {
        for j in 0..w {
            if a_s[i][j] == '#' {
                count += 1;
            }
        }   
    }

    if h+w-1 == count {
        println!("Possible")
    } else {
        println!("Impossible")
    }
}
