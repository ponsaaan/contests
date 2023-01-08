use proconio::input;

fn main() {
    input! {
        n: usize,
        a_s: [i64;n]
    }

    let mut ans = 2000000000;

    let mut cum = vec![0; n];
    cum[0] = a_s[0];
    for i in 1..n {
        cum[i] = cum[i - 1] + a_s[i];
    }

    for i in 0..(n - 1) {
        let x = cum[i];
        let y = cum[cum.len() - 1] - x;
        ans = ans.min((x - y).abs());
    }

    println!("{}", ans);
}
