#[allow(unused_imports)]
use proconio::{input, fastout, marker::Chars};
#[allow(unused_imports)]
use std::collections::{HashMap, HashSet};

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input!{
        A: i64,
        B: i64,
        K: i64,
    }

    let mut map = HashSet::new();
    for i in A..(A + K) {
        if i >= A && i <=B {
            map.insert(i);
        }
    }

    for i in (B - K + 1)..=B {
        if i >= A && i <=B {
            map.insert(i);
        }
    }

    let mut vec:Vec<i64> = map.into_iter().collect();
    vec.sort();


    for i in &vec {
        println!("{}", i);
    }
}
