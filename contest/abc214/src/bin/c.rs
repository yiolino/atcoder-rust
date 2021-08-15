#[allow(unused_imports)]
use proconio::{input, fastout, marker::Chars};
#[allow(unused_imports)]
use std::collections::{HashSet, HashMap, BTreeSet, VecDeque, BinaryHeap};
#[allow(unused_imports)]
use std::cmp::{max, min, Reverse};
#[allow(unused_imports)]
use itertools::Itertools;

#[fastout]
fn main() {
    input!{
        n: usize,
        s: [usize; n],
        t: [usize; n],
    }

    let mut vec = vec![1_000_000_000+1; n];

        // まずはn=1の場合

    let mut time = t[0];
    vec[0] = min(vec[0], time);

    for i in 1..(2*n) {
        let ii = i % n;
        let tmp_catch_time = if ii != 0 {
            time + s[(ii as i64 - 1) as usize]} else {
                time + s[n-1]
            };
        if t[ii] <= tmp_catch_time {
            time = t[ii];
        } else {
            time = tmp_catch_time;
        }
        vec[ii] = min(vec[ii], time);
    }

    for v in vec {
        println!("{}", v);
    }

}
