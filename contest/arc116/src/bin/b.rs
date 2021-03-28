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
        mut _A: [usize; N],
    }

    // let mut ans = 0;
    // for i in 0..1 << N {
    //     let mut vec = vec![];
    //     for j in 0..N {
    //         if i >> j& 1 > 0 {
    //             vec.push(*&A[j])
    //         }
    //     }
        
    //     let mut max = 0;
    //     let mut min = 0;
    //     if let Some(v) = vec.iter().max() {
    //         max = *v;
    //     }
    //     if let Some(v) = vec.iter().min() {
    //         min = *v;
    //     }

    //     ans += max * min;
    //     ans %= 998244353;

    // }

    // println!("{}", ans);
}
