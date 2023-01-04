use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    if 2 * n < m {
        println!("{}", n + (m - 2 * n) / 4);
    } else {
        // cが残ってるのでまだ作れる
        println!("{}", m / 2);
    }
}
