use proconio::input;

fn main() {
    input! {
        n: usize,
        t: usize,
        a: [usize;n]
    }

    let loop_time: usize = a.iter().sum();
    let mut target_time: usize = t % loop_time;
    for (i, x) in a.iter().enumerate().take(n) {
        if target_time < *x {
            println!("{} {}", i + 1, target_time);
            return;
        }
        target_time -= *x;
    }
}
