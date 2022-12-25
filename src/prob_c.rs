use proconio::input;

fn main() {
    input! {
        s:String,
    }

    let mut before_char = 'd';
    let mut target = 0;
    let mut count = 0;

    for i in s.chars() {
        if before_char == '0' {
            if i == '0' {
                if target == 1 {
                    // countしない
                    target = 0;
                } else {
                    count += 1;
                    target += 1;
                }
            } else {
                count += 1;
                target = 0;
            }
        } else {
            if i == '0' {
                target += 1;
            } else {
                target = 0;
            }
            count += 1;
        }
        before_char = i;
    }

    print!("{}\n", count)
}
