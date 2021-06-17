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
        n: usize,
        s: [String; n],
    }

    let mut f = 1;

    for i in 0..n {
        if s[i] == "OR" {
            f = f + 2_i64.pow(1+i as u32);
        }
    }

    println!("{}", f);
}
