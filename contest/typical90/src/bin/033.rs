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
        mut h: usize,
        mut w: usize,
    }

    let ans:usize;

    if h == 1 || w == 1 {
        ans = h * w;
    } else {
        ans = ((h + 1) / 2) * ((w + 1) / 2);
    }

    println!("{}", ans);
}
