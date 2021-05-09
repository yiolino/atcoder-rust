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
        N: i64,
        a: [i64; N],
    }

    let mut map = std::collections::HashMap::new();
    for (i, ai) in a.into_iter().enumerate() {
        let mut vec = vec![];
        vec.push(i as i64 + 1);
        vec.push(ai);
        vec.sort();
        *map.entry(vec).or_insert(0) += 1;
    }

    let mut ans = 0;
    for (_k, v) in map {
        if v == 2 {
            ans += 1;
        }
    }

    println!("{}", ans);
}
