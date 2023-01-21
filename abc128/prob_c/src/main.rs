use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
    }
    // 各電球がどの番号のスイッチに紐づいているか
    let mut target = Vec::new();
    for _ in 0..m {
        input! {
            k: usize,
            s_k: [usize;k]
        }
        target.push(s_k);
    }
    input! {
        p_s: [usize;m]
    }
    let ans = (0..1 << n)
        .filter(|&bit| {
            target.iter().enumerate().all(|(i, s)| {
                let ons_count = s.iter().filter(|&&x| bit >> (x - 1) & 1 == 1).count();
                ons_count % 2 == p_s[i]
            })
        })
        .count();
    println!("{}", ans)
}
