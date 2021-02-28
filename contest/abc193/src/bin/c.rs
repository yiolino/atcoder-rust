#[allow(unused_imports)]
use proconio::{input, fastout, marker::Chars};
#[allow(unused_imports)]
use std::collections::{HashMap, HashSet};

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input!{
        N: i64,
    }

    let mut counter = 0;

    // let mut n = 2;
    let mut set = HashSet::new();
    let rootN = (N as f64).sqrt() as i64;

    for i in 2..=rootN {
        for j in 2..=N {
            if i.pow(j as u32) <= N {
                set.insert(i.pow(j as u32));
            } else {
                break;
            }
        }
    }

    for _ in set {
        counter +=1;
    }
    println!("{}", N - counter);
}
