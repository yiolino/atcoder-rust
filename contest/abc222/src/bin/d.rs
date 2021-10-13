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
        mut a: [usize; n],
        mut b: [usize; n],
    }

    // 0番目を作っておく
    a.insert(0, 0);
    b.insert(0, 0);
    
    let m = *max(a.iter().max().unwrap(), b.iter().max().unwrap());

    let mut dp = vec![vec![0; m+1]; n+1];
    let mut r = vec![vec![0; m+1]; n+1];  // 累積和用のテーブル

    dp[0][0] = 1;
    for i in 0..=m {
        r[0][i] = 1;
    }

    for i in 1..=n {
        for j in 0..=m {
            if a[i] <= j && j <= b[i] {
                    if j == 0 {
                        dp[i][j] = 1;
                        r[i][j] = 1;
                    }
                    dp[i][j] = r[i-1][j] % 998244353;
                }
            
            if j != 0 {
                r[i][j] = (r[i][j-1] + dp[i][j]) % 998244353;
            }
        }
    }

    println!("{}", r[n][m]);
}
