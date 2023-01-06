use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        a: usize,
        b: usize,
        c: usize,
        d: usize,
        s: Chars,
    }

    for i in (a - 1)..c {
        if i + 1 == n {
            break;
        }
        if s[i] == '#' && s[i + 1] == '#' {
            println!("No");
            return;
        }
    }

    for i in (a - 1)..d {
        if i + 1 == n {
            break;
        }
        if s[i] == '#' && s[i + 1] == '#' {
            println!("No");
            return;
        }
    }

    if c < d {
        // switchする必要がない
        println!("Yes");
        return;
    }
    // switchする必要あり
    for i in (b - 1)..d {
        if s[i] != '#' && s[i - 1] == '.' && s[i + 1] == '.' {
            // ３連続.があればswitchできる
            println!("Yes");
            return;
        }
    }

    println!("No")
}
