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
        x: i64,
        y: i64,
    }

    if x != y {
        let ans = 3 - x - y;
        println!("{}", ans);
    } else {
        let ans = x;
        println!("{}", ans);
    }
    
}
