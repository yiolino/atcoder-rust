#[allow(unused_imports)]
use proconio::{input, fastout, marker::Chars};
#[allow(unused_imports)]
use std::collections::{HashSet, HashMap, BTreeSet, VecDeque, BinaryHeap};
#[allow(unused_imports)]
use std::cmp::{max, min, Reverse};
#[allow(unused_imports)]
use itertools::Itertools;

#[fastout]
fn main() {
    input!{
        s: usize,
        t: usize,
    }

    let mut cnt = 0;
    for i in 0..=100 {
        for j in 0..=100 {
            for k in 0..=100 {
                if i + j + k <= s && i * j * k <= t {
                    cnt += 1;
                }
            }
        }
    }

    println!("{}", cnt);
}
