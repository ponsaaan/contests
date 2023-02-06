use num::{integer};
use proconio::input;

fn main() {
    input! {
        n: usize,
        mut p: usize,
    }

    if n == 1 {
        println!("{}", p);
        return;
    }

    let tmp = integer::nth_root(p, n as u32) ;
    let mut vec = Vec::new();
    for i in (1..=tmp).rev() {
        if p >= i &&  p % i.pow(n as u32) == 0 {
            vec.push(i);
            p %= i.pow(n as u32)
        }
    }

    let mut ans = 1;
    for v in vec {
        ans *= v;
    }
    println!("{}", ans)
}
