use num::Integer;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a_s: [usize;n]
    }

    let g = a_s.iter().fold(0, |g, a| g.gcd(a));
    println!("{}", g)
}
