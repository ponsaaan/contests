use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        events: [(usize,usize);q]
    }

    let mut tmp = vec![0;n];

    for i in 0..q {
        let (c, x) = events[i];

        if c == 1 {
            tmp[x-1] += 1;
        } else if c == 2 {
            tmp[x-1] += 2;
        } else {
            if tmp[x-1] >= 2 {
                println!("Yes")
            } else {
                println!("No")
            }
        }
    }
}
