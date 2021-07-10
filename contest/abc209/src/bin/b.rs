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
        x: usize,
        a: [usize; n],   
    }

    let mut sum = 0;

    for i in 0..n {
        if i % 2 == 0 {
            sum += a[i];
        } else {
            sum += a[i] - 1;
        }
    }

    if sum <= x {
        println!("Yes");
    } else {
        println!("No");
    }
}
