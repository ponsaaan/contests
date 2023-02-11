
use proconio::input;

fn main() {
    input! {
        n: usize,
        x_s: [usize;n]
    }

    if x_s.len() == 2 {
        println!("{}", x_s[1]);
        println!("{}", x_s[0]);
        return;
    }

    let mut clone = x_s.clone();
    clone.sort();

    let mut tmp_array = Vec::new();

    let target = clone.len()/2;
    tmp_array.push(clone[target-2]);
    tmp_array.push(clone[target-1]);
    tmp_array.push(clone[target]);
        tmp_array.push(clone[target+1]);

    for x in x_s {
        if x >= tmp_array[2] {
            println!("{}", tmp_array[1])
        } else {
            println!("{}", tmp_array[2])
        }
    }

}
