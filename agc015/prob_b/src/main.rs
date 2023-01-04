use proconio::{input, marker::Chars};

fn main() {
    input! {
        s_s: Chars,
    }

    let s_len = s_s.len();
    let mut cum = vec![0; s_len];
    let mut cnt = 0;
    for i in 0..s_len {
        if s_s[i] == 'U' {
            cnt += 1;
        }
        cum[i] = cnt;
    }

    let cum_max = cum[s_len - 1];

    let mut ans = 0;
    for i in 0..s_len {
        ans += cum_max - cum[i] + s_len - i - 1;
    }

    for i in 0..s_len {
        if s_s[i] == 'U' {
            ans += s_len - i - 1
        } else {
            ans += 2 * (s_len - i - 1);
        }
    }

    println!("{}", ans);
}
