#[allow(unused_imports)]
use proconio::{input, fastout, marker::Chars};
#[allow(unused_imports)]
use std::collections::{HashSet, HashMap, BTreeSet, VecDeque};
#[allow(unused_imports)]
use std::cmp::{max, min};
#[allow(unused_imports)]
use itertools::Itertools;

#[fastout]
fn main() {
    input!{
        n: usize,
        s: Chars,
    }

    let mut vec = vec![];
    let mut cnt = 1;

    for i in 0..n-1 {
        if s[i+1] == s[i] {
            cnt += 1;
        } else {
            vec.push(cnt);
            cnt = 1;
        }
    }
    vec.push(cnt);


    let mut ans = sum_tosa(n);

    for v in vec {
        ans -= sum_tosa(v);
    }

    println!("{}", ans);
}


fn sum_tosa(n: usize) -> usize {
    n * (n + 1) / 2
}