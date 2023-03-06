use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars,
    }

    let mut target_array = vec![0;n];
    let mut init = 0;
    for i in 1..n {
        if s[i] == 'E' {
            init += 1;
        }
    }
    if init == 0 {
        println!("0");
        return;
    }
    target_array[0] = init;
    

    for i in 1..n {
        target_array[i] = target_array[i-1];
        if s[i] == 'W' {
            target_array[i] += 1;
        }
        if s[i-1] == 'E' {
            target_array[i] -= 1;
        }
        if target_array[i] == 0 {
            println!("0");
            return;
        }
    }

    target_array.sort();
    println!("{}", target_array[0]);
}
