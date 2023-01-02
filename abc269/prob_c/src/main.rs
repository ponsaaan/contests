use proconio::input;

fn main() {
    input! {
        n: i64,
    }

    let mut a = vec![];

    for l in 0..60 {
        if n & (1 << l) != 0 {
            a.push(l);
        }
    }
    let k = a.len();

    for i in 0..(1 << k) {
        let mut target_sum: i64 = 0;
        for j in 0..k {
            if i & (1 << j) != 0 {
                target_sum += 1 << a[j];
            }
        }

        println!("{}", target_sum);
    }
}
