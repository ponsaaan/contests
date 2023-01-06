use proconio::input;

fn main() {
    input! {
        w: usize,
        h: usize,
        n: usize,
        xya: [(usize, usize,usize);n]
    }
    let mut lower_left = (0, 0);
    let mut upper_right = (w, h);

    for (x, y, a) in xya {
        match a {
            1 => {
                if upper_right.0 <= x {
                    println!("{}", 0);
                    return;
                }
                // xより左
                if lower_left.0 < x {
                    lower_left.0 = x;
                }
            }
            2 => {
                if lower_left.0 >= x {
                    println!("{}", 0);
                    return;
                }
                // xより右
                if upper_right.0 > x {
                    upper_right.0 = x;
                }
            }
            3 => {
                if upper_right.1 <= y {
                    println!("{}", 0);
                    return;
                }
                // yより下
                if lower_left.1 < y {
                    lower_left.1 = y;
                }
            }
            4 => {
                if lower_left.1 >= y {
                    println!("{}", 0);
                    return;
                }
                // yより上
                if upper_right.1 > y {
                    upper_right.1 = y;
                }
            }

            _ => unreachable!(),
        }
    }
    let ans = (upper_right.0 - lower_left.0) * (upper_right.1 - lower_left.1);
    println!("{}", ans)
}
