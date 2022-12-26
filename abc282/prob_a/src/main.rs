use proconio::input;

fn main() {
    input! {
        k :usize,
    }

    let mut result = String::new();

    for (i, c) in (b'A'..=b'Z')
        .map(|c| (c as char))
        .filter(|c| c.is_ascii_uppercase())
        .enumerate()
    {
        if i >= k {
            break;
        }
        result += &c.to_string()
    }

    println!("{}", result);
}
