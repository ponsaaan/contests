use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a_s: [usize;n]
    }


    let mut vec:Vec<usize> = vec![0;200001];
    let mut set = HashSet::new();
    for a in a_s {
        vec[a] += 1;
        set.insert(a);
    }
    vec.sort();

    let mut ans = 0;
    let mut cnt = 0;
    for i in vec {
        if i == 0 {
            continue;
        }
        if set.len() <=k {
            println!("0");
            return;
        }
        if cnt == set.len()-k {
            break;
        }
        cnt += 1;
        ans += i;
    }
    println!("{}", ans)
    
}
