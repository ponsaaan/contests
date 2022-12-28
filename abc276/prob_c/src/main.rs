use proconio::input;

fn main() {
    input! {
            n:usize,
            mut p:[usize;n],
    }

    let mut j = n - 2;
    while p[j] < p[j + 1] {
        j -= 1;
    }
    let mut k = n - 1;
    while p[j] < p[k] {
        k -= 1;
    }

    p.swap(j, k);
    let mut p2 = p.split_off(j + 1);
    p2.sort();
    p2.reverse();

    for x in &p {
        print!("{} ", x);
    }
    for x in &p2 {
        print!("{} ", x);
    }
    println!();
}
