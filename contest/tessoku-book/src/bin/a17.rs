use std::cmp::min;
use itertools::Itertools;
use proconio::input;

fn main() {
    input!{
        n: usize,
        a: [i32;  n - 1],
        b: [i32; n - 2],
    }

    let mut dp = vec![0_i32; n];
    dp[1] = a[0];

    for i in 2..n {
        dp[i] = min(dp[i - 1] + a[i - 1], dp[i - 2] + b[i - 2]);
    }

    let mut path = vec![];
    let mut place = n - 1;
    path.push(place);
    while place > 0 {
        if dp[place] - a[place - 1] == dp[place - 1] {
            place -= 1;
        } else if dp[place] - b[place - 2] == dp[place - 2] {
            place -= 2;
        }

        path.push(place)
    }

    path.reverse();
    println!("{}", path.len());

    let ans = path.into_iter().map(|x| x + 1).join(" ");
    println!("{}", ans);
}
