use std::cmp;

use proconio::input;

fn main() {
    input! {
        n: u64,
        a: u64,
        b: u64,
        c: u64,
        d: u64,
        e: u64,
    }

    let min = cmp::min(a, cmp::min(b, cmp::min(c, cmp::min(d, e))));

    println!("{}", 4 + (n + min - 1) / min);
}
