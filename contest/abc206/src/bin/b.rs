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
        n: i64,
    }

    let mut cnt = 0;

    for i in 1..1000_000 {
        cnt += i;
        if cnt >= n {
            println!("{}", i);
            return;
        }
    }
}
