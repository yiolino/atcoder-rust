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
        mut A: [i64; N],
    }

    A.sort();

    let mut ans = A[0] + 1;
    for i in 0..N-1 {
        ans *= A[i+1] - A[i] + 1;
        ans %= 1_000_000_000 + 7;
    }

    println!("{}", ans);
}
