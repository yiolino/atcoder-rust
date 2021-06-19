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
        q: usize,
        mut a: [usize; n],
        k: [usize; q],
    }

    // A0を追加する
    a.insert(0, 0);

    let mut c = vec![];
    c.push(0);

    let mut cnt = 0;
    for i in 0..n {
        cnt += a[i+1] - a[i] - 1;  
        c.push(cnt);
    }

    for ki in k {
        if ki > c[n] {
            println!("{}", a[n] + ki - c[n]);

        } else {
            // 二分探索
            let mut lower = 0;
            let mut upper = n;

            while upper - lower > 1 {
                let mid = (lower + upper) / 2;

                if c[mid] >= ki {
                    upper = mid;
                } else {
                    lower = mid;
                }
            }

            println!("{}", a[lower] + ki - c[lower]);
        }
    }
}
