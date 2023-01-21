use num::traits::Pow;
use proconio::input;

fn main() {
    input! {
        n: usize,
        p: usize,
    }

    let tmp = (p as f64).powf(1.0 / (n as f64)) as u64;
    for g in (1..tmp).rev() {
        if p as u32 % (g as u32).pow(n as u32) == 0 {
            println!("{}", g)
        }
    }
}
