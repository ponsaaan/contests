use proconio::input;

fn main() {
    input! {
        x: u64,
    }
    let pre = x / 11;

    if x % 11 == 0 {
        println!("{}", pre * 2);
    } else if x % 11 <= 6 {
        println!("{}", pre * 2 + 1);
    } else {
        println!("{}", pre * 2 + 2);
    }
}
