use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [String;n],
    }

    let mut count = 0;

    for i in 0..n {
        if i == n - 1 {
            break;
        }
        for j in i + 1..n {
            let mut result = true;
            for k in 0..m {
                if s[i].chars().nth(k).unwrap() != 'o' && s[j].chars().nth(k).unwrap() != 'o' {
                    result = false;
                    break;
                }
            }
            if result {
                count += 1;
            }
        }
    }

    println!("{}", count);
}
