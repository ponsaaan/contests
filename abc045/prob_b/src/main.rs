use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut sa: Chars,
        mut sb: Chars,
        mut sc: Chars,
    }

    let mut teban:usize = 0; // a: 0, b: 1, c: 2

    loop {
        match teban {
            0 => {
                if sa.len() == 0 {
                    println!("A");
                    return;
                }
                match sa[0] {
                    'a' => {
                        teban = 0;
                    },
                    'b' => {
                        teban = 1;
                    },
                    'c' => {
                        teban = 2;
                    },
                    _ => unreachable!()
                }
                sa.remove(0);
            },
            1 => {
                if sb.len() == 0 {
                    println!("B");
                    return;
                }
                match sb[0] {
                    'a' => {
                        teban = 0;
                    },
                    'b' => {
                        teban = 1;
                    },
                    'c' => {
                        teban = 2;
                    },
                    _ => unreachable!()
                }
                sb.remove(0);
            },
            2 => {
                if sc.len() == 0 {
                    println!("C");
                    return;
                }
                match sc[0] {
                    'a' => {
                        teban = 0;
                    },
                    'b' => {
                        teban = 1;
                    },
                    'c' => {
                        teban = 2;
                    },
                    _ => unreachable!()
                }
                sc.remove(0);
            },
            _ => unreachable!()
        }

    }


}
