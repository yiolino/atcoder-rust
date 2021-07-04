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
        mut p: i64,
    }

    let f = [1, 2, 6, 24, 120, 720, 5040, 40320, 362880, 3628800];

    let mut cnt = 0;
    let mut idx = 9;

    while p > 0 {
        while f[idx] > p {
            idx -= 1;
        }

        p -= f[idx];
        cnt += 1;
    }

    println!("{}", cnt);
}
