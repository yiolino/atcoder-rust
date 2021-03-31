#[allow(unused_imports)]
use proconio::{input, fastout, marker::Chars};
#[allow(unused_imports)]
use std::collections::{HashSet, HashMap, BTreeSet};
#[allow(unused_imports)]
use std::cmp::{max, min};

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input!{
        R: usize,
        C: usize,
        SB: [[usize; C]; R],
    }

    let mut ans = 0;
    for bit in 0..1 << R {
        let mut tmp = 0;
        for c in 0..C {
            let mut col_sum = 0;
            for r in 0..R {
                if bit >> r & 1 > 0 {
                    col_sum += SB[r][c] ^ 1;  // XORで反転
                } else {
                    col_sum += SB[r][c];
                }
            }
            tmp += max(col_sum, R - col_sum);
        }

        ans = ans.max(tmp);
    }

    println!("{}", ans);
}
