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

const MOD: i64 = 998244353;

#[fastout]
fn main() {
    input!{
        n: usize,
        a: [usize; n],
    }

    // dpで解く
    let mut dp = vec![vec![0_i64; 10]; n];

    dp[0][a[0]] = 1;

    for i in 0..n-1 {
        for j in 0..=9 {
            if dp[i][j] != 0 {
                let plus = (j + a[i + 1]) % 10;
                let mutip = (j * a[i + 1]) % 10;
                dp[i + 1][plus] += dp[i][j] % MOD;
                dp[i + 1][mutip] += dp[i][j] % MOD;

                dp[i + 1][plus] = (dp[i + 1][plus] + MOD) % MOD;
                dp[i + 1][mutip] = (dp[i + 1][mutip] + MOD) % MOD;
            }
        }
    }


    for i in 0..10 {
        println!("{}", dp[n-1][i]);
    }
}
