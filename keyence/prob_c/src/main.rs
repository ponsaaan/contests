use std::cmp;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a_s: [u64;n],
        mut b_s: [u64;n],
    }

    let a_sum:u64 = a_s.iter().sum();
    let b_sum :u64= b_s.iter().sum();

    if a_sum< b_sum {
        println!("-1");
        return;
    }

    // 差分の配列
    let mut diffs = Vec::new();
    // マイナスの個数の総和
    let mut cnt_minus = 0;
    for i in 0..n {
        diffs.push(a_s[i] as i64 - b_s[i] as i64);
        if a_s[i] < b_s[i] {
            cnt_minus += 1;
        }
    }
    if cnt_minus == 0 {
        println!("0");
        return;
    }
    
    // マイナスの総和
    let mut minus_sum:i64 = diffs.iter().filter(|&&item| item < 0).sum();
    // プラスの大きい順から、マイナスの総和が相殺されるまで減らしていく
    diffs.sort_by_key(|&x| cmp::Reverse(x));
    let mut cnt_plus = 0;
    for i in diffs {
        if i > 0 {
            minus_sum += i;
            cnt_plus += 1;
            if minus_sum >= 0 {
                break;
            }
        }
    }


    println!("{}", cnt_minus+cnt_plus)

}
