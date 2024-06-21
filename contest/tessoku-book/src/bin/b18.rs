use std::vec;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        s: usize,
        a: [usize; n]
    }

    // 通常のDPを行う
    let mut dp = vec![vec![false; s + 1]; n + 1];
    dp[0][0] = true;

    for i in 1..n + 1 {
        let ai = a[i - 1];
        for j in 0..s + 1 {
            if dp[i - 1][j] {
                dp[i][j] = true;
            }
            if ai <= j  && dp[i - 1][j - ai] {
                dp[i][j] = true;
            }
        }
    }

    if !dp[n][s] {
        println!("{}", -1);
        return;
    }

    let mut ans_vec = vec![];
    let mut place = s;  // 現在の総和
    for i in (1..n+1).rev() {
        if dp[i-1][place] {
            continue;  // iのカードは選ばなくても良い
        }

        let ai = a[i - 1];
        if dp[i-1][place - ai] {
            ans_vec.push(i);  // iを選ぶ
            place -= ai;
        }
    }

    ans_vec.reverse();

    let ans = ans_vec.iter()
        .map(|x| x.to_string())
        .join(" ");

    println!("{}", ans_vec.len());
    println!("{}", ans);
}
