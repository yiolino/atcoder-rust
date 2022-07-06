use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut ans = vec![0; n];
    for i in 0..n-1 {
        if a[i] > a[i + 1] {
            ans[i] ^= 1;
            ans[i + 1] ^= 1;
        }
    }

    println!("{}", ans.iter().join(" "));
}
