use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        mut x_s: [isize;m],
    }

    if n >= m {
        println!("{}", 0);
        return;
    }
    x_s.sort();
    let mut cum = vec![0; m - 1];
    for i in 0..m - 1 {
        cum[i] = x_s[i + 1] - x_s[i];
    }

    cum.sort();
    let mut ans = 0;
    for i in 0..m - n {
        ans += cum[i];
    }
    println!("{}", ans);
}
