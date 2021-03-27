#[allow(unused_imports)]
use proconio::{input, fastout, marker::Chars};
#[allow(unused_imports)]
use std::collections::{HashSet, HashMap, BTreeSet};
#[allow(unused_imports)]
use std::cmp::{max, min};

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input!{
        N: usize,
        K: usize,
        mut h: [i64; N],
    }
 
    h.sort();
 
    let mut ans = std::i64::MAX;
    
    for i in 0..=N-K {
        ans = min(ans, h[K-1+i as usize] - h[0+i as usize]);
    }
 
    println!("{}", ans);
}
