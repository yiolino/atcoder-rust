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
        d: i64,
        n: i64,
        m: i64,
        mut dn: [i64; n - 1],
        k: [i64; m],
    }

    // 本店を加える。この時、本店は距離が0の店と、dの店の2通りあると考えれば良い。
    dn.push(0);
    dn.push(d);
    dn.sort();  // sortして2分探索に持ち込む。

    let mut ans = 0;
    for ki in k {
        let mut left = 0;
        let mut right = n;
        while right - left > 1 {
            let mid = (left + right) / 2;
            if ki <= dn[mid as usize] {
                right = mid;
            } else {
                left = mid;
            }
        }
        
        ans += min(dn[right as usize] - ki, ki - dn[left as usize]);
    }

    println!("{}", ans);
}
