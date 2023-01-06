use proconio::input;

fn main() {
    input! {
        x: isize,
        y: isize,
    }

    let mut ans = 0;
    if x.abs() <= y.abs() {
        ans += y.abs() - x.abs();
        ans += isize::from(x < 0);
        ans += isize::from(y < 0);
    } else {
        ans += x.abs() - y.abs();
        ans += isize::from(x > 0);
        ans += isize::from(y > 0);
    }

    println!("{}", ans);
}
