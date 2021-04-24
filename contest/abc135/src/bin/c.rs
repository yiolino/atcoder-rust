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
        mut A: [i64; N+1],
        mut B: [i64; N],
    }

    let mut ans = 0;

    for i in 0..N {
        let k = min(B[i], A[i]);
        ans += k;
        B[i] -= k;

        let k = min(A[i+1], B[i]);
        ans += k;
        A[i+1] -= k;
    } 

    println!("{}", ans);
}
