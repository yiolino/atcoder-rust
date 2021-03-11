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
        h: [i64; N],
    }

    let mut vec = vec![1000_000_000_000_000; N as usize];

    vec[0] = 0;
    vec[1] = (&h[1] - &h[0]).abs();
    for i in 2..N {
        vec[i as usize] = min((&h[i as usize] - &h[(i-1) as usize]).abs() + vec[(i-1) as usize], 
                                vec[i as usize]);
        vec[i as usize] = min((&h[i as usize] - &h[(i-2) as usize]).abs() + vec[(i-2) as usize],
                                vec[i as usize]);
    }

    println!("{}", &vec[(N-1) as usize]);
}
