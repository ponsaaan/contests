use proconio::input;

fn main() {
    input! {
        n: usize,
        a_s: [isize;n]
    }

    let mut sum = 0;
    for i in 0..n {
        if i == 0 {
            sum += a_s[i].abs();
            continue;
        }

        sum += (a_s[i] - a_s[i-1]).abs();
    }

    // 最後に0に戻る分
    sum += a_s[n-1].abs();

    for i in 0..n {
        let mut ans = sum;
        let mut i_minus_1 = 0;
        let mut i_plus_1 = 0;
        if i >= 1 {
            i_minus_1 = a_s[i-1]
        }
        if i+1 < n {
            i_plus_1 = a_s[i+1]
        }
        ans = ans - (a_s[i] - i_minus_1).abs() - (i_plus_1 - a_s[i]).abs();
        ans += (i_plus_1 - i_minus_1).abs();
        println!("{}", ans);
    }


}
