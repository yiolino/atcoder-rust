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
        HS: [(i64, i64); N],
    }

    let mut lower = 0;
    let mut upper = 1_000_000_000_000_000;

    while upper - lower > 1 {
        let mid = (lower + upper) / 2;
        let mut nokori_jikan = vec![];
        let mut is_ok = true;


        for i in 0..N {
            let heights = HS[i].0;
            if mid < heights {
                is_ok = false;
            }
            
            let velo = HS[i].1; 
            let nokori = (mid - heights) / velo;
            nokori_jikan.push(nokori);
        }

        nokori_jikan.sort();

        for j in 0..N {
            if j as i64 > nokori_jikan[j] {
                is_ok = false;
            }
        }

        if is_ok {
            upper = mid;
        } else {
            lower = mid;
        }
    }

    println!("{}", upper);
}
