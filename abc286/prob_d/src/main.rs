use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        ab: [(usize,usize);n]
    }

    let mut dp = vec![vec![false;x+1];n];
    let target_a0 = ab[0].0;
    let target_b0 = ab[0].1;
    for j in 0..target_a0 * target_b0+1 {
        if j > x {
            break;
        }
        if j % target_a0 == 0 {
            dp[0][j] = true
        }
    }
    for i in 1..n {
        let target_a = ab[i].0;
        let target_b = ab[i].1;
        for j in 0..x+1 {
            if dp[i-1][j] {
                dp[i][j] = true;
                continue;
            }
            for k in 0..target_b+1 {
                if j < k*target_a {
                    continue;
                }
                if dp[i-1][j - k*target_a] {
                    dp[i][j] = true;
                }
            }
        }
    }

    for i in 0..n {
        if dp[i][x] {
            println!("Yes");
            return;
        }
    }
    println!("No")
}
