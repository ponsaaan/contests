use proconio::input;

fn main() {
    input! {
        n: u64,
        k: u64,
        s: u64,
    }

    let mut ans = Vec::new();
    for _ in 0..k {
        ans.push(s);
    }
    if s == 1000000000 {
        for _ in k..n {
            ans.push(1);
        }
    } else {
        for _ in k..n {
            ans.push(1000000000);
        }
    }

    for a in ans {
        print!("{} ", a);
    }
}
