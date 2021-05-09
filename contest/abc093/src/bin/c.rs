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
        mut vec: [i64; 3],
    }

    let mut cnt = 0;
    while !(vec[0] == vec[1] && vec[1] == vec[2]) {
        vec.sort();

        let diff_ab = vec[1] - vec[0];
        let diff_bc = vec[2] - vec[1];

        if diff_bc >= diff_ab {
            vec[2] -= 1;
        } else {
            vec[0] += 2;
        }
        cnt += 1;
    }

    println!("{}", cnt);
}
