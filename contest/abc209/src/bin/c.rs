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
        n: i64,
        mut c: [i64; n],
    }

    let waru = 1_000_000_000 + 7;

    c.sort();
    
    let mut ans = 1;
    for i in 0..n {
        ans *= c[i as usize] - i;
        ans %= waru;
    }

    println!("{}", ans);
}
