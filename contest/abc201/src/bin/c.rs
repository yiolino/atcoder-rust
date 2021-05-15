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
        S: Chars,
    }

    let mut ans = 0;

    for i in 0..10000 {
        let mut flg = vec![false; 10];
        let mut now = i;

        for _j in 0..4 {
            flg[now % 10] = true;
            now /= 10;
        }

        let mut flg2 = true;
        for j in 0..10 {
            if S[j] == 'o' && !flg[j] {
                flg2 = false;
            }
            if S[j] == 'x' && flg[j] {
                flg2 = false;
            }
        }

        ans += flg2 as i64;
    }

    println!("{}", ans);
}
