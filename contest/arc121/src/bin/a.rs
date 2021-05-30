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
        N: usize,
    }

    let mut vec = vec![];

    for i in 0..N {
        input!{
            x: i64,
            y: i64,
        }
        vec.push((i, x, y));
    }

    let mut ans = 0;

    // xの値でsort
    vec.sort_by_key(|chank| chank.1);
    ans = ans.max(vec[N - 1].1 - vec[1].1);
    ans = ans.max(vec[N - 2].1 - vec[0].1);
    let tmp_idx = (vec[0].0, vec[N - 1].0);
    let tmp_ans =  vec[N-1].1 - vec[0].1;

    // yの値でsort
    vec.sort_by_key(|chank| chank.2);
    ans = ans.max(vec[N - 1].2 - vec[1].2);
    ans = ans.max(vec[N - 2].2 - vec[0].2);

    let idx = (vec[0].0, vec[N - 1].0);
    if idx.0 + idx.1 != tmp_idx.0 + tmp_idx.1 &&
        idx.0 * idx.1 != tmp_idx.0 * tmp_idx.1 {
            let tmp = min(vec[N-1].2 - vec[0].2, tmp_ans);
            ans = ans.max(tmp);
        }
        

    println!("{}", ans);
}
