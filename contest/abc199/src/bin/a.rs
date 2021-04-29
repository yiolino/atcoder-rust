#[allow(unused_imports)]
use proconio::{input, fastout, marker::Chars};
#[allow(unused_imports)]
use std::collections::{HashSet, HashMap, BTreeSet};
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use itertools::Itertools;

#[allow(non_snake_case)]
fn main() {
    input!{
        A: i64,
        B: i64,
        C: i64,
    }

    let ans = A * A + B * B;

    if ans < C * C {
        println!{"Yes"};
        return;
    } else {
        println!{"No"};
        return;
    }
}
