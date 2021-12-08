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


fn main() {
    input!{
        n: usize,
        a: [usize; n],
    }

    // dp[l][r] = [l, r) l以上r未満のスライムを1体にするときのコストの最小値
    // 右は閉区間であることに注意
    let mut dp = vec![vec![0; n+1]; n+1];


    // 累積和を持っておく
    let mut cum = vec![vec![0; n+1]; n+1];
    for i in 0..n {
        cum[i][i+1] = a[i];
        for j in i+2..=n {
            cum[i][j] = cum[i][j-1] + a[j-1];
        }
    }
    
    for len in 2..=n {
        for l in 0..=n-len {
            let r = l+len;
            dp[l][r] = std::usize::MAX;

            for m in l+1..r {
                dp[l][r] = dp[l][r].min(cum[l][r] + dp[l][m] + dp[m][r]);
            }

        }
    }

    println!("{}", dp[0][n]);
}
