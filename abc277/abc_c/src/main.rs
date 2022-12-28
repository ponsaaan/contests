use std::collections::{BTreeSet, HashMap, VecDeque};

use proconio::input;

fn main() {
    input! {
        n: u32,
        xs: [(u32,u32);n],
    }

    let mut hash_map: HashMap<u32, Vec<u32>> = HashMap::new();

    for (a, b) in xs {
        hash_map.entry(a).or_default().push(b);
        hash_map.entry(b).or_default().push(a);
    }

    let mut set: BTreeSet<u32> = BTreeSet::new();
    let mut queue: VecDeque<u32> = VecDeque::new();

    queue.push_back(1);
    set.insert(1);

    while let Some(u) = queue.pop_front() {
        if let Some(vs) = hash_map.get(&u) {
            for v in vs {
                if !set.contains(v) {
                    set.insert(*v);
                    queue.push_back(*v);
                }
            }
        }
    }
    let max = set.iter().last().unwrap();
    println!("{}", max);
}
