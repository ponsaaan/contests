use proconio::input;

fn main() {
    input! {
        n: usize,
        mut b_s: [usize;n]
    }
    let mut ans = Vec::new();
    while !b_s.is_empty() {
        for i in (0..b_s.len()).rev() {
            if i + 1 == b_s[i] {
                let tmp = b_s.remove(i);
                ans.push(tmp);
                break;
            }
            if i == 0 {
                println!("{}", -1);
                return;
            }
        }
    }

    for &a in ans.iter().rev() {
        println!("{}", a);
    }
}
