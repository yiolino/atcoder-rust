#[allow(unused_imports)]
use proconio::{input, fastout, marker::Chars};
#[allow(unused_imports)]
use std::collections::{HashSet, HashMap, BTreeSet};
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use itertools::Itertools;


#[fastout]
#[allow(non_snake_case)]
fn main() {
    input!{
        N: usize,
        P: [i64; N],
        Q: [i64; N],
    }

    let perms = (1..=N).permutations(N);
    let mut vec = vec![];

    for p in perms {
        vec.push(p.iter().
            map(|x| x.to_string()).
            collect::<Vec<_>>().
            join(""));
    }

    vec.sort();
    let p = P.iter().
            map(|x| x.to_string()).
            collect::<Vec<_>>().
            join("");
    let q = Q.iter().
            map(|x| x.to_string()).
            collect::<Vec<_>>().
            join("");

    let mut num_p = 100000;
    let mut num_q = 100000;

    for (i, v) in vec.iter().enumerate() {
        if *v == p {
            num_p = (i + 1) as i64;
        }
        if *v == q {
            num_q = (i + 1) as i64;
        }
    }

    println!("{}", (num_p - num_q).abs());
}
