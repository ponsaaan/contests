use std::vec;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        xy: [(usize,usize);m]
    }

    let mut target_vec = vec![1; n];
    let mut target_red = vec![false; n];
    target_red[0] = true;

    for (x, y) in xy {
        if target_red[x - 1] {
            // redの可能性のある箱が選択された
            target_red[y - 1] = true;
        }
        target_vec[x - 1] -= 1;
        target_vec[y - 1] += 1;
        if target_vec[x - 1] == 0 {
            target_red[x - 1] = false;
        }
    }

    println!("{}", target_red.iter().filter(|&x| *x).count());
}
