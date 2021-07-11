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
        k: usize,
    }

    let mut vec = vec![];

    for _ in 0..n {
        input!{
            a: usize,
            b: usize,
        }

        vec.push(b);
        vec.push(a - b);
    }

    vec.sort();
    vec.reverse();

    let mut ans = 0;
    for i in 0..k {
        ans += vec[i];
    }

    println!("{}", ans);
}
