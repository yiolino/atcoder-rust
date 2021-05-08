#[allow(unused_imports)]
use proconio::{input, fastout, marker::Chars};
#[allow(unused_imports)]
use std::collections::{HashSet, HashMap, BTreeSet, VecDeque};
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use itertools::Itertools;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input!{
        N: usize,
        A: [i64; N],
    }

    let mut AA = vec![];
    for a in A {
        AA.push(a % 200);
    }

    let mut map = std::collections::HashMap::new();
    for a in AA {
        *map.entry(a).or_insert(0) += 1_i64;
    }

    let mut ans = 0;
    for (_k, v) in map {
        if v > 1 {
            for i in 1..v {
                ans += i;
            }
        }
    }

    println!("{}", ans);
}
