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
        A: [usize; N],
        B: [usize; N],
        C: [usize; N],
    }

    let mut mapA = std::collections::HashMap::new();
    for a in A {
        *mapA.entry(a).or_insert(0) += 1i64;
    }

    let mut mapC = std::collections::HashMap::new();
    for c in C {
        *mapC.entry(c).or_insert(0) += 1i64;
    }

    let mut ans = 0;
    for (i, b) in B.iter().enumerate() {
        let mut tmp = 0;

        if mapA.contains_key(b) {
                tmp += mapA[b];
            }

        if mapC.contains_key(&(i+1) as &usize) {
            tmp *= mapC[&(i+1) as &usize];
            ans += tmp;
        }
    }

    println!("{}", ans);
}
