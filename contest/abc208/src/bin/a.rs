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
        a: usize,
        b: usize,
    }


    if b > 6 * a {
        println!("No");
    } else if a > b {
        println!("No");
    } else {
        println!("Yes");
    }
}
