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
        N: i64,
    }

    let mut counter = 0;
    
    for i in 1..=N {
        if i % 2 == 0 {
            continue;
        }
        let mut set = HashSet::new();
        let mut cnt = 1;
        while cnt <= i*i {
            if i % cnt == 0 {
                set.insert(cnt);
                set.insert(i / cnt);
            }
            cnt += 1;
        }
        if set.len() == 8 {
            counter += 1;
        }
    }

    println!("{}", counter);
}
