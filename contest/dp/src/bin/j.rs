#[allow(unused_imports)]
use proconio::{input, fastout, marker::Chars, marker::Usize1, marker::Bytes};
#[allow(unused_imports)]
use std::collections::{HashSet, HashMap, BTreeSet, VecDeque, BinaryHeap};
#[allow(unused_imports)]
use std::cmp::{max, min, Reverse};
#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use petgraph::unionfind::UnionFind;
#[allow(unused_imports)]
use superslice::Ext;

#[fastout]
fn main() {
    input!{
        n: usize,
    }

    // dp[i][j][k] := 寿司1個の皿がi枚、2個の皿がj枚、3個の皿がk枚ある状態から、寿司を全て
    //                  なくすのに必要な操作回数の「期待値」とする

    // まず初期状態で 1~3個の皿が何個あるかカウントする
    let mut one = 0;
    let mut two = 0;
    let mut three = 0;
    for _ in 0..n {
        input!(a: i64);
        if a == 1 {
            one += 1;
        } else if a == 2 {
            two += 1;
        } else {
            three += 1;
        }
    }

    // dp[onw][two][three] が求める答え
    let mut dp = vec![vec![vec![-1.0; 310]; 310]; 310];

    let ans = rec(one, two, three, &mut dp, n);

    println!("{}", ans);
}


fn rec(i: usize, j: usize, k: usize, dp: &mut Vec<Vec<Vec<f64>>>, n: usize) -> f64 {
    if dp[i][j][k] >= 0.0 {
        return dp[i][j][k]
    }
    if i == 0 && j == 0 && k == 0 {
        return 0.0;
    }

    let mut res = 0.0;
    if i > 0 {
        res += rec(i.wrapping_sub(1), j, k, dp, n) * i as f64;
    }
    if j > 0 {
        res += rec(i+1, j.wrapping_sub(1), k, dp, n) * j as f64;
    }
    if k > 0 {
        res += rec(i, j+1, k.wrapping_sub(1), dp, n) * k as f64; 
    }
    res += n as f64;
    res /= (i + j + k) as f64;

    dp[i][j][k] = res;
    res
}