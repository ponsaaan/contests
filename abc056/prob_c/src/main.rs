use proconio::input;

fn main() {
    input! {
        x: u64,
    }

    let mut time_vec = vec![0u64; 1000000];
    time_vec[0] = 0;
    for i in 1..1000000 {
        let target_time = time_vec[i - 1] + i as u64;
        if x <= target_time {
            println!("{}", i);
            return;
        }
        time_vec[i] = target_time;
    }
}
