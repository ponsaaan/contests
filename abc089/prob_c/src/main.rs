use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s_s: [Chars;n]
    }

    let mut count_vec: Vec<u64> = vec![0; 5];

    for s in s_s {
        let target = s[0];
        match target {
            'M' => count_vec[0] += 1,
            'A' => count_vec[1] += 1,
            'R' => count_vec[2] += 1,
            'C' => count_vec[3] += 1,
            'H' => count_vec[4] += 1,
            _ => {}
        }
    }

    let mut ans = 0;

    for i in 0..count_vec.len() - 2 {
        for j in i + 1..count_vec.len() - 1 {
            if i == j {
                continue;
            }
            for k in j + 1..count_vec.len() {
                if i == k || j == k {
                    continue;
                }
                ans += count_vec[i] * count_vec[j] * count_vec[k];
            }
        }
    }

    println!("{}", ans)
}
