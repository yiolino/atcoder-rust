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
        n: usize,
        xy: [(i64, i64); n],
    }

    let mut set = HashSet::new();

    for &xy in &xy {
        set.insert(xy);
    }

    let mut ans = 0;

    for i in 0..n {
        for j in 0..n {
            let (x1, y1) = &xy[i];
            let (x2, y2) = &xy[j];

            let (x3, y3) = (x2 - (y2 - y1), y2 + x2 - x1);
            let (x4, y4) = (x3 - (x2- x1), y3 - (y2 - y1));

            if set.contains(&(x3, y3)) && set.contains(&(x4, y4)) {
                ans = ans.max((x2 - x1)*(x2 - x1) + (y2 - y1)*(y2 - y1));
            }

        }
    }

    println!("{}", ans);
}
