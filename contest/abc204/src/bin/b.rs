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
        A: [i64; N],
    }

    let mut ans = 0;

    for i in 0..N {
        if A[i] >= 10 {
            ans += A[i] - 10;
        }
    }

    println!("{}", ans);
}
