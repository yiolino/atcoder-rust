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
        N: usize,
        M: usize,
        A: [[usize; M]; N],
    }

    let mut max_score = 0;

    for i in 0..M-1 {
        for j in i+1..M {
            let mut tmp_score = 0;
            for k in 0..N {
                tmp_score += max(A[k][i], A[k][j]);
            }
            if tmp_score > max_score {
                max_score = tmp_score;
            }
        }
    }


    println!("{}", max_score);
}
