use proconio::input;

fn main() {
    input! {
        n: usize,
        mut p_s:[usize;n]
    }

    if n == 2 && p_s[0] == 1 {
        println!("{}", 1);
        return;
    }

    let mut ans = 0;
    for i in 0..(n - 2) {
        if p_s[i] == i + 1 {
            p_s.swap(i, i + 1);
            ans += 1;
        } else if p_s[i + 1] == i + 2 && p_s[i + 2] == i + 3 {
            p_s.swap(i + 1, i + 2);
            ans += 1;
        }
    }

    if p_s[p_s.len() - 2] == p_s.len() - 1 || p_s[p_s.len() - 1] == p_s.len() {
        ans += 1;
    }

    println!("{}", ans);
}
