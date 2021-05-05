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
        X: i64,
        Y: i64,
    }

    let Z = Y / X;

    let mut a = 2;

    let mut counter = 0;
    while a <= Z {
        a *= 2;
        counter += 1;
    }

    println!("{}", counter + 1);
}
