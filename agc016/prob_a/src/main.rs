use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let azchars = [
        'a', 'b', 'c', 'd', 'e', 'f', 'g', 'h', 'i', 'j', 'k', 'l', 'm', 'n', 'o', 'p', 'q', 'r',
        's', 't', 'u', 'v', 'w', 'x', 'y', 'z',
    ];

    let mut ans = 100;
    for &c in &azchars {
        let mut t = s.clone();
        if !t.contains(&c) {
            continue;
        }

        let mut loop_cnt = 0;
        while !t.iter().all(|&x| x == c) {
            for i in 0..t.len() - 1 {
                if t[i] == c || t[i + 1] == c {
                    t[i] = c;
                }
            }
            t.pop();
            loop_cnt += 1;
        }
        ans = ans.min(loop_cnt);
    }
    println!("{}", ans)
}
