use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    }

    println!("{}", (n - 1 + k - 2) / (k - 1));
}
