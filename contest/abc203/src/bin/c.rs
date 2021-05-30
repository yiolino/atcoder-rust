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
        K: i64,
        mut AB: [(i64, i64); N],
    }

    let mut map = HashMap::new();

    for ab in AB {
        *map.entry(ab.0).or_insert(0) += ab.1;
    }

    let mut vec = vec![];

    for (k, v) in map {
        vec.push((k, v));
    }

    vec.sort();

    let mut now = 0;
    let mut money = K;

    for (a, b) in vec {
        if a > money + now {
            now += money;
            println!("{}", now);
            return;
        }
        else {
            now += money;
            money = b; 
        }
    }

    let ans = now + money;

    println!("{}", ans);
}
