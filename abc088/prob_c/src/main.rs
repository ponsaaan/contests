use proconio::input;

fn main() {
    input! {
        c11: isize,
        c12: isize,
        c13: isize,
        c21: isize,
        c22: isize,
        c23: isize,
        c31: isize,
        c32: isize,
        c33: isize,
    }

    // 列ごと
    if c12-c11 != c22-c21 || c22-c21 != c32-c31 {
        println!("No");
        return;
    }
    if c13-c12 != c23-c22 || c23-c22 != c33-c32 {
        println!("No");
        return;
    }

    // 行ごと
    if c21-c11 != c22-c12 || c22-c12 != c23-c13 {
        println!("No");
        return;
    }
    if c31-c21 != c32-c22 || c32-c22 != c33-c23 {
        println!("No");
        return;
    }

    println!("Yes");
}
