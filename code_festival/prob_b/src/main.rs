use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut sum = 0;
    let mut vec = Vec::new();
    for i in 1..=10000000 {
        sum += i;
        vec.push(i);
        if sum > n {
            break;
        }   
    }

    for v in vec {
        if sum - v == n {
            continue;
        }
        println!("{}", v)
    }
}
