use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
    }

    let m = 1000_000_007;

    let ans = power(a, b, m);
    println!("{}", ans);


}

// aのb乗をmで割った余り
fn power(a: usize, b:usize, m: usize) -> usize {
    let mut p = a;
    let mut ans = 1;
    for i in 0..30 {
        if b & (1<<i) != 0 {
            ans = ans * p % m;
        }
        p = p * p % m;
    }
    return ans;
}