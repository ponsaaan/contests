use num::Integer;
use proconio::input;

fn main() {
    input! {
        n: usize,
        t_s: [u64;n]
    }

    let mut ans = t_s[0];

    for i in 1..n {
        ans = ans.lcm(&t_s[i]);
    }

    println!("{}", ans)
}
