use num::integer::gcd;
use proconio::input;

fn main() {
    input! {
        n: usize,
        x: isize,
        x_s: [isize;n],
    }

    let mut ans = 0;

    for i in x_s {
        ans = gcd(ans, num::abs(i - x))
    }

    println!("{}", ans);
}
