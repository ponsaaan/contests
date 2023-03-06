use proconio::input;

fn main() {
    input! {
        n: isize,
        a: isize,
        b: isize,
        c: isize,
        d: isize,
    }

    if (b - c).abs() > 1 {
        println!("No");
        return;
    }

    let mut ans = 0;
    if b == 0 && c == 0 {
        if a > 0 && d > 0 {
            println!("No");
            return;
        }
        if a > 0 {
            ans += a+1;
        }
        if d > 0 {
            ans += d+1;
        }

        if ans <= n {
            println!("Yes");
        } else {
            println!("No");
        }
        return;
    }

    let mut ans = 0;

    if b > c {
        // xy..の並び
        ans += 2*b;
        if a > 0 {
            ans += a;
        }
        if d > 0 {
            ans += d;
        }
        
    } else {
        // yx...の並び
        ans += 2*c;
        if a > 0 {
            ans += a;
        }
        if d > 0 {
            ans += d;
        }
    }

    if ans <= n {
        println!("Yes");
    } else {
        println!("No");
    }
    
}
