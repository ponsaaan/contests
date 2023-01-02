use proconio::input;

fn main() {
    input! {
        n: usize,
        a_s: [usize;n]
    }

    if n <= 2 {
        println!("{}", 1);
        return;
    }

    let mut ans = 1;
    let mut flag = 0; // 0:リセット 1:増加 2:減少

    for i in 1..n {
        match flag {
            0 => {
                if a_s[i - 1] < a_s[i] {
                    flag = 1;
                } else if a_s[i - 1] > a_s[i] {
                    flag = 2;
                }
            }
            1 => {
                if a_s[i - 1] > a_s[i] {
                    ans += 1;
                    flag = 0;
                }
            }
            2 => {
                if a_s[i - 1] < a_s[i] {
                    ans += 1;
                    flag = 0;
                }
            }
            _ => unreachable!(),
        }
    }

    println!("{}", ans);
}
