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
        q: usize,
        tx: [[usize; 2]; q],
    }

    let mut d = std::collections::VecDeque::new();

    for i in 0..q {
        let t = tx[i][0];
        let x = tx[i][1];

        if t == 1 {
            d.push_front(x);
        } else if t == 2 {
            d.push_back(x);
        } else {
            println!("{}", d[x-1]);
        }
    }
}
