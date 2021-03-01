#[allow(unused_imports)]
use proconio::{input, fastout, marker::Chars};
#[allow(unused_imports)]
use std::collections::{HashSet, HashMap, BTreeSet};

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input!{
        X: i64,
    }

    let mut max = 0;
    let rootX = (X as f64).sqrt() as i64;


    for i in 0..=rootX {
        for j in 0..=rootX {
            let cand = i.pow(j as u32);
            if cand > X {
                break;
            } else if cand > max {
                max = cand;
            }
        }
    }

    println!("{}", max);
}
