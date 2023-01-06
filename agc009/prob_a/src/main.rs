use proconio::input;

fn main() {
    input! {
        n: usize,
        ab: [(i64,i64);n]
    }

    let mut a_s = Vec::new();
    let mut b_s = Vec::new();

    for (a, b) in ab {
        a_s.push(a);
        b_s.push(b);
    }

    let mut count = 0;
    for i in (0..n).rev() {
        // これまでに押されたボタンの数だけ足す
        a_s[i] += count;
        if a_s[i] == 0 {
            continue;
        }
        let mut target = 0;
        if a_s[i] < b_s[i] {
            target = b_s[i] - a_s[i];
        } else if a_s[i] % b_s[i] == 0 {
            continue;
        } else {
            target = b_s[i] - a_s[i] % b_s[i];
        }
        a_s[i] += target;
        count += target;
    }

    println!("{}", count);
}
