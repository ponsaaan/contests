use proconio::{input, marker::Chars};

fn main() {
    input! {
        t: usize
    }

    

    for i in 0..t {
        input! {
           nk: (usize,usize),
           s: Chars,
        }

        let n = nk.0;
        let k = nk.1;


        if k < n {
            // 前からk個削った値が回文かどうか
            let target1 = &s[k..n];
            let mut ans = true;
            for i in 0..n-k {
                if target1[i] != target1[n-k-i-1] {
                    ans = false;
                }
            }
            // 後からk個削った値が回文かどうか
            let target2 = &s[0..n-k];
            for i in 0..n-k {
                if target2[i] != target2[n-k-i-1] {
                    ans = false;
                }
            }
            if ans {
                println!("Yes")
            } else {
                println!("No")
            }
            continue;
        } else if n==k {
           println!("Yes");
           continue;
        } else {
            if n% 3 != 0 {
                println!("No")
            } else {
                let target = n/3;
                let tmp_array = &s[target..n-target];
                if s[0..target] == s[n-target..n] {
                    let mut tmp = true;
                    for j in 0..n/3 {
                        if s[j] != tmp_array[(n/3)-j-1] {
                            tmp = false;
                        }
                    }
                    if tmp {
                        println!("Yes")
                    } else {
                        println!("No")
                    }
                } else {
                    println!("No")
                }
            }
        }
    }
}
