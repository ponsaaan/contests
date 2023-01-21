use proconio::input;

fn main() {
    input! {
        n: usize,
        p: usize,
        q: usize,
        r: usize,
        _: usize,
        mut a_s: [usize;n],
    }

    for i in p..q + 1 {
        a_s.swap(i - 1, i - 1 + (r - p));
    }

    for i in a_s {
        print!("{} ", i)
    }
}
