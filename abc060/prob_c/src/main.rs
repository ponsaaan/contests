use proconio::input;

fn main() {
    input! {
        n: usize,
        t: usize,
        ts: [usize;n]
    }

    let mut ans = 0;

    for i in 1..n {
        if t <= ts[i] - ts[i - 1] {
            ans += t;
        } else {
            ans += ts[i] - ts[i - 1];
        }
    }
    ans += t;

    println!("{}", ans);
}
