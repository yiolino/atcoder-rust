#[allow(unused_imports)]
use proconio::{input, fastout, marker::Chars};
#[allow(unused_imports)]
use std::collections::{HashSet, HashMap, BTreeSet};
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use itertools::Itertools;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input!{
        N: usize,
        A: [i64; N],
        B: [i64; N],
    }

    let mut minimal = 0;
    let mut maximam = 10000;

    for i in 0..N {
        minimal = minimal.max(A[i]);
        maximam = maximam.min(B[i]);
    }

    let mut ans = maximam - minimal + 1;
    if ans < 0 {
        ans = 0;
    }
    println!("{}", ans);
}
