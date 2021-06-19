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
        n: f64,
    }

    let price = (n * 1.08) as i64;

    if price < 206 {
        println!("Yay!");

    } else if price == 206 {
        println!("so-so");

    } else {
        println!(":(");
    }
}
