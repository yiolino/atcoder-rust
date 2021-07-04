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
     mut k: usize,
    }

    let mut a = vec![];

    for i in 0..n {
        let mut v = vec![];
        input! {
            ai: usize,
        }

        v.push(ai);
        v.push(i);
        v.push(0);

        a.push(v);
    }
    
    // aiでsort
    a.sort_by_key(|s| s[0]);

    let common = k / n;

    k = k % n;


    for i in 0..k {
        a[i][2] += 1;
    }

    a.sort_by_key(|s| s[1]);


    for i in 0..n {
        println!("{}", a[i][2] + common)
    }
}
