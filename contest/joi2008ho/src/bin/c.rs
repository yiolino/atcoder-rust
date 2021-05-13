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
        N: i64,
        M: i64,
        mut target: [i64; N],
    }

    target.push(0);

    let mut set = HashSet::new();

    for ti in &target {
        for tj in &target {
            let tmp = ti + tj;
            set.insert(tmp);
        }
    }

    let mut vec = set.into_iter().collect::<Vec<_>>();
    vec.sort();

    let mut ans = 0;

    for i in &vec {
        let mut lower = 0;
        let mut upper = vec.len();

        while upper - lower > 1 {
            let mid = (upper + lower) / 2;
            if i + vec[mid as usize] <= M {
                lower = mid;
                ans = ans.max(*i as i64 + vec[mid as usize]);
            } else {
                upper = mid;
            }
        }
    }


    println!("{}", ans);
}
