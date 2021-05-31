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
        D: [[i64; N]; N],
        Q: usize,
        P: [usize; Q],
    }

    // 二次元累積和のテーブル
    // まずN＋1 * N+1 のテーブルを作る
    let mut S = vec![vec![0; N + 1]; N + 1];
    for i in 0..N {
        for j in 0..N {
            S[i + 1][j + 1] = S[i + 1][j] + S[i][j + 1] - S[i][j] + D[i][j];
        }
    }


    // すべての長方形区域の面積を集計
    // val[v] := 面積が v の長方形領域の総和の最大値
    let mut val = vec![0; N * N + 1];
    for x1 in 0..N {
        for x2 in x1+1..=N {
            for y1 in 0..N {
                for y2 in y1+1..=N {
                    let area = (x2 - x1) * (y2 - y1);
                    let sum = S[x2][y2] - S[x1][y2] - S[x2][y1] + S[x1][y1];
                    val[area] = max(val[area], sum);
                }
            }
        }
    }


    // 集計。 val[v] := 面積が v 「以下の」長方形領域の最大値（v全部使わないでも良い）
    for i in 0..N * N {
        val[i+1] = max(val[i+1], val[i]); 
    }

    // クエリに答える
    for p in P {
        println!("{}", val[p]);
    }
}
