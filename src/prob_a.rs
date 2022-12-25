use proconio::input;
fn main() {
    input! {
        param:[u32;2],
    }

    let a = param[0];
    let b = param[1];

    println!("{}", a.pow(b));
}
