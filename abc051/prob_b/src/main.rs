use proconio::input;

fn main() {
    input! {
        k: usize,
        s: usize,
    }
    let mut ans = 0;

    for i in 0..=k {
        for j in 0..=k {
            if s >= i + j && s - i - j <= k {
                ans += 1;
            }
        }
    }

    println!("{}", ans);
}
