use proconio::input;

fn main() {
    input! {
        n: isize,
        m: isize,
    }

    if (m - n).abs() > 1 {
        println!("{}", 0);
        return;
    }

    let mut ans = 1;
    for i in 1..=n {
        ans *= i;
        ans %= 1000000007;
    }
    for i in 1..=m {
        ans *= i;
        ans %= 1000000007;
    }

    if (m - n).abs() == 1 {
        println!("{}", ans)
    } else {
        println!("{}", (2 * ans) % 1000000007);
    }
}
