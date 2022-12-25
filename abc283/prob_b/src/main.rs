use proconio::input;
fn main() {
    input! {
        n:u32,
        mut a:[usize;n],
        q: u32,
    }

    for _ in 0..q {
        input! {
            first: u32,
        }

        match first {
            1 => {
                input! {
                    k: usize,
                    x: usize,
                }
                // 変換
                a[k - 1] = x;
            }
            2 => {
                input! {
                    k: usize,
                }
                // print
                println!("{}", a[k - 1])
            }
            _ => (),
        }
    }
}
