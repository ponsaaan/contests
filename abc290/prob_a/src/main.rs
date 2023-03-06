use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a_s: [usize;n],
        b_s: [usize;m]
    }

    let mut ans = 0;
    for i in b_s {
        ans += a_s[i-1]
    }
    
    println!("{}", ans )
}
