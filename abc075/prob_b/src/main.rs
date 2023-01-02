use std::vec;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        h:usize,
        w: usize,
        mut ss: [Chars; h]
    }

    let dx: Vec<isize> = vec![-1, 0, 1];
    let dy: Vec<isize> = vec![-1, 0, 1];

    for x in 0..h {
        for y in 0..w {
            if ss[x][y] == '.' {
                // 8方向を見て+1する
                let mut count = 0;
                for k in &dx {
                    for l in &dy {
                        if *k == 0 && *l == 0 {
                            continue;
                        }
                        let xx = x as isize + *k;
                        let yy = y as isize + *l;
                        if xx >= 0
                            && yy >= 0
                            && (xx as usize) < h
                            && (yy as usize) < w
                            && ss[xx as usize][yy as usize] == '#'
                        {
                            count += 1;
                        }
                    }
                }
                ss[x][y] = std::char::from_digit(count as u32, 10).unwrap();
            }
        }
    }

    for x in 0..h {
        for y in 0..w {
            print!("{}", ss[x][y]);
        }
        println!()
    }
}
