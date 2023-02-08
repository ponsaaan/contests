use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        d_s: [usize;n],
        m: usize,
        t_s: [usize;m],
    }

    if n < m {
        println!("NO");
        return;
    }

    let mut map_d: HashMap<usize, usize> = HashMap::new();

    for i in 0..n {
        let target = d_s[i];
        if map_d.contains_key(&target) {
            map_d.insert(target, *map_d.get(&target).unwrap()+1);
        } else {
            map_d.insert(target, 1);
        }
    }

    for i in 0..m {
        let target = t_s[i];

        if !map_d.contains_key(&target) {
            println!("NO");
            return;
        }

        let tmp = *map_d.get(&target).unwrap();
        if tmp == 0 {
            println!("NO");
            return;
        }
        map_d.insert(target, *map_d.get(&target).unwrap()-1);
    }

    println!("YES")
}
