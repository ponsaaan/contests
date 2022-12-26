use proconio::input;

fn main() {
    input! {
        n: usize,
        mut s: String,
    }

    let mut result = String::new();
    let mut is_on_to_update = true;

    for c in s.chars() {
        match c {
            ',' => {
                if is_on_to_update {
                    result += ".";
                } else {
                    result += &c.to_string();
                }
            }
            _ => {
                result += &c.to_string();
                if c == '"' {
                    is_on_to_update = !is_on_to_update;
                }
            }
        }
    }

    println!("{}", result);
}
