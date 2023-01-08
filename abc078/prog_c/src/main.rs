use proconio::input;

fn main() {
    input! {
        n: u32,
        m: u32,
    }

    let time_by_one = (n - m) * 100 + m * 1900;

    println!("{}", time_by_one * 2u32.pow(m));
}
