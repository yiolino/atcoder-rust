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
        a: i64,
        b: i64,
        c: i64,
    }

    let ans = 7 - a + 7 - b + 7 - c;

    println!("{}", ans);
}
