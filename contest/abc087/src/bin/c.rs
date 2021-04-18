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
        A1: [usize; N],
        A2: [usize; N],
    }

    let mut ans = 0;
    for i in 0..N {
        let mut tmp = 0;
        for j in 0..=i {
            tmp += A1[j];
        }
        for j in i..N {
            tmp += A2[j];
        }

        ans = ans.max(tmp);
    }

   println!("{}", ans);
}
