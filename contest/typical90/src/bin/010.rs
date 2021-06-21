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
        cp: [[usize; 2]; n],
        q: usize,
        lr: [[usize; 2]; q],
    }

    // 累積和で考える
    let mut vec_cumsum_c1 = vec![];
    vec_cumsum_c1.push(0);
    let mut cumsum_c1 = 0;
    let mut vec_cumsum_c2 = vec![];
    vec_cumsum_c2.push(0);
    let mut cumsum_c2 = 0;

    for i in 0..n {
        if cp[i][0] == 1 {
            cumsum_c1 += cp[i][1];
        } else {
            cumsum_c2 += cp[i][1];
        }

        vec_cumsum_c1.push(cumsum_c1);
        vec_cumsum_c2.push(cumsum_c2);
    }


    for i in 0..q {
        let l = lr[i][0];
        let r = lr[i][1];

        let ans_c1 = vec_cumsum_c1[r] - vec_cumsum_c1[l-1];
        let ans_c2 = vec_cumsum_c2[r] - vec_cumsum_c2[l-1];

        println!("{} {}", ans_c1, ans_c2);
    }
}
