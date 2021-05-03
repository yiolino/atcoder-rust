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
    input! {
        N: usize,
        ABCDE: [[usize; 5]; N],
    }

    let mut lower = 0;
    let mut upper = 1000_000_001;

    'next: while upper - lower > 1 {

        let mid = (lower + upper) / 2;
        let mut set = HashSet::new();

        for abcde in &ABCDE {
            let mut x = 0;
            for (i, &e) in abcde.iter().enumerate() {
                if e >= mid {
                    x |= 1 << i;
                } 
            }

            set.insert(x);
        }

        for i in &set {
            for j in &set {
                for k in &set {
                    if i | j | k == (1 << 5) - 1 {
                        lower = mid;
                        continue 'next;
                    }
                }
            }
        }

        upper = mid;
    }

    println!("{}", lower);
}