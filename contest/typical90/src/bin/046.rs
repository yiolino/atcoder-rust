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
        mut a: [usize; n],
        mut b: [usize; n],
        mut c: [usize; n],
    }

    // 余りベースで考える。
    let mut cnt_a = vec![0_usize; 46];
    let mut cnt_b = vec![0_usize; 46];
    let mut cnt_c = vec![0_usize; 46];

    for i in 0..n {
        a[i] %= 46;
        cnt_a[a[i]] += 1;

        b[i] %= 46;
        cnt_b[b[i]] += 1;

        c[i] %= 46;
        cnt_c[c[i]] += 1;
    }

    let mut ans = 0;
    for i in 0..46 {
        for j in 0..46 {
            for k in 0..46 {
                if (i + j + k) % 46 == 0 {
                    ans += cnt_a[i] * cnt_b[j] * cnt_c[k];
                }
            }
        }
    }

    println!("{}", ans);
}
