use proconio::input;

fn main() {
    input! {
        n: usize,
        a_s: [usize;n]
    }

    let mut cnt_4 = 0;
    let mut cnt_2 = 0;

    for a in a_s {
        if a % 4 == 0 {
            cnt_4 += 1
        } else if a % 2 == 0 {
            cnt_2 += 1;
        }
    }

    let target = cnt_4*2;
    if n % 2 == 0 {
        // 偶数
        if target >= n {
            println!("Yes");
            return;
        }
        if n-target <= cnt_2 {
            println!("Yes")
        } else {
            println!("No")
        }
    } else {
        // 奇数
        if target >= n || target + 1 == n {
            println!("Yes");
            return;
        }
        if n-target <= cnt_2 {
            println!("Yes")
        } else {
            println!("No")
        }
    }
}
