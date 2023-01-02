use proconio::input;

fn main() {
    input! {
        r: usize,
        g: usize,
        b: usize,
        n: usize,
    }

    let mut ans = 0;

    for i in 0..n + 1 {
        for j in 0..n + 1 {
            if n < r * i + g * j {
                continue;
            }
            if (n - r * i - g * j) % b == 0 {
                ans += 1;
            }
        }
    }

    println!("{}", ans)
}
