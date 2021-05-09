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
        A: i64,
        B: i64,
        C: i64,
    }

    let fh = A / 2;
    let lh = A - fh;
    let mut ans = (fh * B * C - lh * B * C).abs();

    let fh = B / 2;
    let lh = B - fh;
    ans = ans.min((fh * A * C - lh * A * C).abs());

    let fh = C / 2;
    let lh = C - fh;
    ans = ans.min((fh * A * B - lh * A * B).abs());


    println!("{}", ans);
}
