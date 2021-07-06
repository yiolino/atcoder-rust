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
        a: i64,
        b: u32,
        c: i64,
    }

    let res = c.pow(b) - a;

    let ans: &str;

    if res > 0 {
        ans = "Yes";
    } else {
        ans = "No";
    }

    println!("{}", ans);
}
