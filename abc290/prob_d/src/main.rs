use proconio::input;

fn main() {
    input! {
        t: usize,
        ndk: [(usize,usize,usize);t]
    }

    for (n,d,k) in ndk {
        // あまりを求める
        let target = k % d;
        if n % d == 0 {
            // 余り0,1,2...で順番になる
            println!("{}",(d * (k-1))%n);
        } else {
            // イレギュラー
        }
    }
}
