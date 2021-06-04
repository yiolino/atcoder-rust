#[allow(unused_imports)]
use proconio::{input, fastout, marker::Chars};
#[allow(unused_imports)]
use std::collections::{HashSet, HashMap, BTreeSet, VecDeque};
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use itertools::Itertools;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input!{
        N: usize,
        K: usize,
        A: [[i64; N]; N],
    }

    let center = K*K - K*K/2;

    let mut high = 1000_000_000_000_000;
    let mut low = -1;

    while high - low > 1 {
        let mid = (high + low) / 2;  // 中央値は mid 以下か？という問
        // mid 以下が K*K / 2 + 1 個以上なら Yes、未満なら No

        let mut tmp_A = vec![vec![0; N];N];
        for ii in 0..N {
            for jj in 0..N {
                tmp_A[ii][jj] = if A[ii][jj] <= mid {1} else {0};
            }
        }

        // K*K 区画の中の1の個数 累積2乗和 tmp_Aより1行1列大きい区画で考える
        let mut S = vec![vec![0; N+1]; N+1];
        for ii in 0..N {
            for jj in 0..N {
                S[ii+1][jj+1] = S[ii][jj+1] + S[ii+1][jj] - S[ii][jj] + tmp_A[ii][jj];
            }
        }

        let mut is_ok = false;
        for i in 0..N-K+1 {
            for j in 0..N-K+1 {
                let mut now = S[i+K][j+K];
                now -= S[i][j+K];
                now -= S[i+K][j];
                now += S[i][j];
                if now >= center {
                    is_ok = true;
                }
            }
        }

        if is_ok {
            high = mid;
        } else{
            low = mid;
        }
    }

    println!("{}", high);
}
