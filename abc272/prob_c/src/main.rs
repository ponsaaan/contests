use proconio::input;
use std::cmp;
fn main() {
    input! {
        n: usize,
        mut xs: [usize;n]
    }

    let mut target_even = vec![];
    let mut target_odd = vec![];

    for i in xs {
        if i % 2 == 0 {
            target_even.push(i);
        } else {
            target_odd.push(i);
        }
    }

    target_even.sort_by_key(|&x| cmp::Reverse(x));
    target_odd.sort_by_key(|&x| cmp::Reverse(x));

    if target_even.len() < 2 && target_odd.len() < 2 {
        println!("{}", -1);
        return;
    }

    if target_even.len() < 2 {
        let sum: usize = target_odd[..2].iter().sum();
        println!("{}", sum);
        return;
    } else if target_odd.len() < 2 {
        let sum: usize = target_even[..2].iter().sum();
        println!("{}", sum);
        return;
    }

    let sum1: usize = target_even[..2].iter().sum();
    let sum2: usize = target_odd[..2].iter().sum();

    let max = cmp::max(sum1, sum2);

    println!("{}", max)
}
