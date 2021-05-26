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
        _M: usize,
        S: [Chars; N],
    }

    let mut odd:i64 = 0;
    let mut even:i64 = 0;

    for i in 0..N {
        let tmp:i64 = S[i].iter()
                            .map(|&s| s as i64 - 48)
                            .sum();

        if tmp % 2 == 0 {
            even += 1;
        } else {
            odd += 1;
        }
    }

    let ans = odd * even;

    println!("{}", ans);
}
