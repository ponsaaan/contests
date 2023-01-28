use proconio::{input, marker::Chars};

const AAA: u64 = 998244353;

fn main() {
    input! {
        n: usize,
        a: Chars,
        b: Chars,
    }

    // aが大きい方とする
    let mut tmp_a = Vec::new();
    let mut tmp_b = Vec::new();
    // 片方を考えうる最小の数にする
    for i in 0..n {
        if a[i] > b[i] {
            tmp_a.push(a[i]);
            tmp_b.push(b[i]);
        } else {
            tmp_b.push(a[i]);
            tmp_a.push(b[i]);
        }
    }

    let mut tmp_10 = Vec::new();
    tmp_10.push(1);

    for i in 1..n {
        tmp_10.push((tmp_10[i - 1] * 10) % AAA);
    }
    let mut num_a = 0;
    let mut num_b = 0;
    for i in 0..n {
        let target = (n - i) - 1;
        num_a += (tmp_a[target] as u64 - 48) * (tmp_10[i]);
        num_b += (tmp_b[target] as u64 - 48) * (tmp_10[i]);
    }

    println!("{}", ((num_a % AAA) * (num_b % AAA)) % AAA)
}
