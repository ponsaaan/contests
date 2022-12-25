use std::collections::HashSet;

use proconio::input;
fn main() {
    input! {
        s: String,
    }

    let mut st: Vec<HashSet<char>> = Vec::new();
    let mut set: HashSet<char> = HashSet::new();
    st.push(HashSet::new());
    for c in s.chars() {
        match c {
            '(' => st.push(HashSet::new()),
            ')' => {
                let remove_target = st.pop().unwrap();
                for target in remove_target {
                    set.remove(&target);
                }
            }
            _ => {
                if set.contains(&c) {
                    println!("No");
                    return;
                }
                st.last_mut().unwrap().insert(c);
                set.insert(c);
            }
        }
    }

    println!("Yes");
}
