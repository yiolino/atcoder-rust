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
        s: usize,
        ab: [(usize, usize); n],
    }

    let mut dp = vec![vec![false; s+1]; n+1];
    dp[n][s] = true;

    for i in (1..=n).rev() {
        for j in (0..=s).rev() {
            if dp[i][j] {
                let a = j.checked_sub(ab[i.wrapping_sub(1)].0);
                let b = j.checked_sub(ab[i.wrapping_sub(1)].1);

                if let Some(v_a) = a {
                    dp[i.wrapping_sub(1)][v_a] = true;
                }
                if let Some(v_b) = b {
                    dp[i.wrapping_sub(1)][v_b] = true;
                }

            }
        }
    }

    if dp[0][0] {
        let mut pos = 0;

        for i in 1..=n {
            let a = ab[i.wrapping_sub(1)].0;
            let b = ab[i.wrapping_sub(1)].1;

            let flg = dp[i].get(pos + a);
            if let Some(f) = flg {
                if *f {
                    pos += a;
                    print!("A");
                    continue;
                }
            }

            let flg = dp[i].get(pos + b);
            if let Some(f) = flg {
                if *f {
                    pos += b;
                    print!("B");
                    continue;
                }
            }
        }
        println!();
    
    } else {
        println!("Impossible");
    }

}
