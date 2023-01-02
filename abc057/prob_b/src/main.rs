use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(isize,isize);n],
        cd: [(isize,isize);m],
    }

    for &(a, b) in &ab {
        let mut tmp_distance = 400000000;
        let mut target_index = 0;
        for (i, &(c, d)) in cd.iter().enumerate() {
            let target_distance = (a - c).abs() + (b - d).abs();
            if target_distance == tmp_distance {
                continue;
            }
            if tmp_distance > target_distance {
                tmp_distance = target_distance;
                target_index = i;
            }
        }

        println!("{}", target_index + 1);
    }
}
