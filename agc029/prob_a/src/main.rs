use proconio::input;

fn main() {
    input! {
        s:String,
    }
    let mut ans: u64 = 0;
    let mut tmp: u64 = 0;

    for c in s.chars().rev() {
        if c == 'B' {
            ans += tmp;
        } else {
            tmp += 1;
        }
    }

    println!("{}", ans);
}
