use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
    }

    if a > b {
        println!("{}", 0);
        return;
    }

    if n == 1 {
        if a != b {
            println!("{}", 0);
            return;
        }

        println!("{}", 1);
        return;
    }

    let min = (n - 1) * a + b;
    let max = (n - 1) * b + a;

    println!("{}", max - min + 1);
}
