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
        X: Chars,
    }

    let mut i = 0;
    for (j, c) in X.iter().enumerate() {
        if *c == '.' {
            i = j;
        }
    }

    if i == 0 {
        i = X.len();
    }

    let mut vec = vec![];

    for j in 0..i {
        vec.push(X[j]);
    }

    let ans = vec.into_iter().collect::<String>();

    println!("{}", ans);
}
