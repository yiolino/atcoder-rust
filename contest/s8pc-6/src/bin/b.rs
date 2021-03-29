#[allow(unused_imports)]
use proconio::{input, fastout, marker::Chars};
#[allow(unused_imports)]
use std::collections::{HashSet, HashMap, BTreeSet};
#[allow(unused_imports)]
use std::cmp::{max, min};

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input!{
        N: usize,
        AB: [[i64; 2]; N],
    }

    let mut ans = std::i64::MAX;

    for i in 0..N {
        for j in 0..N {
            let mut tmp = 0;

            let enter = AB[i][0];
            let exit = AB[j][1];

            for k in 0..N {
                tmp += (enter - AB[k][0]).abs() + (AB[k][0] - AB[k][1]).abs() + (AB[k][1] - exit).abs();
            }

            ans = ans.min(tmp);
        }
    }

    println!("{}", ans);
}
