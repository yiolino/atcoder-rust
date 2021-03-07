#[allow(unused_imports)]
use proconio::{input, fastout, marker::Chars};
#[allow(unused_imports)]
use std::collections::{HashSet, HashMap, BTreeSet};

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input!{
        N: i64,
        AB: [[i64; 2]; N],
    }

    let mut ans = 1000_000_000_000;

    for i in 0..N {
        for j in i+1..N {
            let cand = std::cmp::max(AB[i as usize][0], AB[j as usize][1]);
            ans = std::cmp::min(ans, cand);
        }
    }

    for i in 0..N {
        for j in i+1..N {
            let cand = std::cmp::max(AB[i as usize][1], AB[j as usize][0]);
            ans = std::cmp::min(ans, cand);
        }
    }

    for i in 0..N {
        let cand = AB[i as usize][0] + AB[i as usize][1];
        ans = std::cmp::min(ans, cand);
    }

    println!("{}", ans);
}
