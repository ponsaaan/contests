use proconio::input;

fn main() {
    input! {n: usize}

    let mut tmp = Vec::new();

    for i in 1..n {
        let sqrt_i = (f64::sqrt(i as f64)) as usize;
        let mut tmp_value = 0;
        for j in 1..sqrt_i+1 {
            if i % j == 0 {
                tmp_value += 1;
            }
        }
        tmp_value = tmp_value * 2;
        if sqrt_i * sqrt_i == i {
            tmp_value -=1;
        }
        tmp.push(tmp_value);
    }

    let mut ans = 0;
    for i in 0..n-1 {
        ans += tmp[i] * tmp[n-i-2];
    }

    println!("{}", ans);
}
