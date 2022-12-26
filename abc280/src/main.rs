use proconio::input;

fn main() {
    input! {
        s: String,
        t: String,
    }

    let s_vec: Vec<char> = s.chars().collect();
    let t_vec: Vec<char> = t.chars().collect();

    for i in 0..s.len() {
        if s_vec[i] != t_vec[i] {
            println!("{}", i + 1);
            return;
        }
    }

    // 当てはまらなければ必ず最後に挿入されている
    println!("{}", t.len())
}
