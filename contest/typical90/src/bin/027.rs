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
        n: usize,
        s: [String; n],
    }

    let mut set = HashSet::new();

    for i in 0..n {
        if !set.contains(&s[i]) {
            set.insert(&s[i]);
            println!("{}", i+1);
        }
    }
}
