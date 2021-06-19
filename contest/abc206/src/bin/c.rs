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
        a: [usize; n],
    }


    let mut map = std::collections::HashMap::new();

    for i in 0..n {
        *map.entry(a[i]).or_insert(0) += 1;
    }

    let vec:Vec<usize> = map.into_iter().map(|(_k, v)| v).collect();

    let mut memo = vec![];

    let mut cnt = 0;
    for i in (1..vec.len()).rev() {
        cnt += vec[i];
        memo.push(cnt);
    }

    memo.reverse();

    let mut ans = 0;
    for i in 0..vec.len()-1 {
        ans += vec[i] * memo[i]
    }


    println!("{}", ans);
}
