use proconio::input;

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
    }

    let mut ans_array = vec![false;n];

    for i in 0..n {
        if i < a {
            ans_array[i] = false;
            continue;
        }
        if i < b {
            ans_array[i] = !ans_array[i-a];
            continue;
        }

        ans_array[i] = !ans_array[i-a] || !ans_array[i-b]
    }

    if ans_array[n-1] {
        println!("First")
    } else {
        println!("Second")
    }
}
