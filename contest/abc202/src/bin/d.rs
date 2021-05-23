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
        mut A: i64,
        mut B: i64,
        mut K: i64,
    }

    // パスカルの三角形をテーブルとして作る
    let mut comb = vec![vec![0; 61]; 61];  // 可変配列の確保
    comb[0][0] = 1_i64;
    for i in 0..60 {
        for j in 0..i+1 {
            comb[i+1][j] += comb[i][j];
            comb[i+1][j+1] += comb[i][j];
        }
    }

    // 先頭から文字を付け足していく
    let mut ans = String::new();
    while A + B > 0 {
        let mut x: i64 = 0;
        if A > 0 {
            x = comb[(A + B - 1) as usize][(A - 1) as usize];
        }
        // Kがxより小さければaを先頭につける。そうでなければ、bをつける
        if K <= x {
            ans = ans + &'a'.to_string();
            A -= 1;
        } else {
            ans = ans + &'b'.to_string();
            B -= 1;
            K -= x;
        }
    }

    println!("{}", ans);
}
