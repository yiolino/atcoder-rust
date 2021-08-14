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
        l: usize,
        k: usize,
        a: [usize; n],
    }

    // 配列aを隣との差の配列に変える。
    let mut b = vec![];
    b.push(a[0]);
    for i in 0..n-1 {
        b.push(a[i+1] - a[i]);
    }
    b.push(l as usize - a[n-1] as usize);

    // 二分探索
    let mut lower = 0;
    let mut higher = 1_000_000_000 + 1;

    while higher - lower > 1 {
        let mid = (higher + lower) / 2;

        let mut cnt = 0;
        let mut tmp = 0;

        for i in 0..=n {
            tmp += b[i];

            if tmp >= mid {
                cnt += 1;
                tmp = 0;
            }
        }

        if cnt >= k + 1 {
            lower = mid;
        } else {
            higher = mid;
        }
        
    }


    println!("{}", lower);
}
