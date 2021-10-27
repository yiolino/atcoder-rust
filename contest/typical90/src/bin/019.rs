#[allow(unused_imports)]
use proconio::{input, fastout, marker::Chars};
#[allow(unused_imports)]
use std::collections::{HashSet, HashMap, BTreeSet, VecDeque};
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use itertools::Itertools;

#[fastout]
fn main() {
    input!{
        n: usize,
        a: [i32; 2*n],
    }

    let mut dp = vec![vec![0; 2*n]; 2*n];
    
    // まず隣り合う同士を引き抜く時を全て埋めてしまう。
    for i in 0..(2*n-1) {
        dp[i][i+1] = (a[i] - a[i+1]).abs();
    }

    for i in (3..2*n).step_by(2) {
        for j in 0..(2*n - i) {
            let mut val = (a[j] - a[j+i]).abs() + dp[j+1][(j+i).wrapping_sub(1)];
            for k in (j+1..(j+i)).step_by(2) {
                val = val.min(dp[j][k] + dp[k+1][j+i]);
            }
            dp[j][j+i] = val;
        }
    }
    
    let ans = dp[0][2*n -1];
    println!("{}", ans);
}

