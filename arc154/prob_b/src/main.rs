use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
        t: Chars,
    }
    if s == t {
        println!("{}", 0);
        return;
    }

    let mut tmp_s = Vec::new();
    let mut tmp_t = Vec::new();
    for i in 0..n {
        tmp_s.push(s[i]);
        tmp_t.push(t[i]);
    }

    tmp_s.sort();
    tmp_t.sort();
    if tmp_s != tmp_t {
        println!("-1");
        return;
    }
    tmp_s.

    let mut s_index_hash = HashMap::new();
    let mut t_index_hash = HashMap::new();


    for i in 0..n {
        s_index_hash.insert(s[i], i);
        s_index_hash.insert(t[i], i);
    }

    for i in 0..n {
        // sの先頭i+1文字を削る
        // tからの該当文字列を削除
        // s==tかどうか
    }

    println!("{}", n - max_cnt);
}
