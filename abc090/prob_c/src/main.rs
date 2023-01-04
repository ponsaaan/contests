use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }

    if n == 1 && m == 1 {
        println!("{}", 1);
        return;
    } else if n == 1 {
        println!("{}", m - 2);
        return;
    } else if m == 1 {
        println!("{}", n - 2);
        return;
    }

    println!("{}", (n - 2) * (m - 2));
}
