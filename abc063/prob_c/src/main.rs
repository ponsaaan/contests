use proconio::input;

fn main() {
    input! {
        n: usize,
        mut ss: [usize;n]
    }

    ss.sort();

    let sum: usize = ss.iter().sum();

    if sum % 10 != 0 {
        println!("{}", sum);
        return;
    }

    for s in ss {
        if s % 10 != 0 {
            println!("{}", sum - s);
            return;
        }
    }

    println!("{}", 0);
}
