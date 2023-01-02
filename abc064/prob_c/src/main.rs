use proconio::input;
use std::cmp;

fn main() {
    input! {
        n: usize,
        mut a_s: [usize;n]
    }

    if n == 1 {
        print!("{}", 1);
        println!(" {}", 1);
    }

    a_s.sort();

    let mut ash: usize = 0;
    let mut brown: usize = 0;
    let mut green: usize = 0;
    let mut water: usize = 0;
    let mut blue: usize = 0;
    let mut yellow: usize = 0;
    let mut orange: usize = 0;
    let mut red: usize = 0;
    let mut other: usize = 0;

    for a in a_s {
        if a <= 399 {
            ash += 1;
        } else if a <= 799 {
            brown += 1;
        } else if a <= 1199 {
            green += 1;
        } else if a <= 1599 {
            water += 1;
        } else if a <= 1999 {
            blue += 1;
        } else if a <= 2399 {
            yellow += 1;
        } else if a <= 2799 {
            orange += 1;
        } else if a <= 3199 {
            red += 1;
        } else {
            other += 1;
        }
    }

    let mut min = 0;
    if ash > 0 {
        min += 1;
    }
    if brown > 0 {
        min += 1;
    }
    if green > 0 {
        min += 1;
    }
    if water > 0 {
        min += 1;
    }
    if blue > 0 {
        min += 1;
    }
    if yellow > 0 {
        min += 1;
    }
    if orange > 0 {
        min += 1;
    }
    if red > 0 {
        min += 1;
    }

    if min == 0 {
        print!("{}", 1);
    } else {
        print!("{}", min);
    }
    println!(" {}", min + other);
}
