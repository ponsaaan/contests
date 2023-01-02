use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    }

    let mut ans = 0.0;

    for i in 1..n + 1 {
        if i >= k {
            ans += 1.0 / (n as f64);
            continue;
        }
        let mut i_edited = i;
        let mut tmp_pow = 1.0;
        // 何回目でkを超えるか
        for _ in 1..100 {
            i_edited *= 2;
            tmp_pow *= 0.5;
            if i_edited >= k {
                break;
            }
        }
        ans += tmp_pow / (n as f64)
    }

    println!("{}", ans);
}
