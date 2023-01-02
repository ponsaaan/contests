use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
    }

    for i in 0..b {
        if a * i % b == c {
            println!("YES");
            return;
        }
    }

    println!("NO");
}
