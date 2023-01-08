use libm::{acos, atan2};
use proconio::input;

fn main() {
    input! {
        a: f64,
        b: f64,
        x: f64,
    }

    let v: f64 = a * a * b;

    if x >= v / 2.0 {
        let theta = atan2(2.0 * (a * b - x / a) / a, a);
        println!("{}", 360.0 * theta / (2.0 * acos(-1.0)));
    } else {
        let theta = atan2(b, 2.0 * x / (a * b));
        println!("{}", 360.0 * theta / (2.0 * acos(-1.0)));
    }
}
