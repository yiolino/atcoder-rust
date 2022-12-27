use std::cmp::min;

use proconio::input;

fn main() {
    input!{
        n: usize,
        a: [usize; n - 1],
        b: [usize; n - 2],
    }

    let mut dp = vec![0; n];

    for i in 1..n {
        if i == 1 {
            dp[i] = a[i - 1];
            continue;
        }

        dp[i] = min(dp[i - 1] + a[i - 1], dp[i - 2] + b[i - 2]);
    }

    println!("{}", dp[n - 1]);
}
