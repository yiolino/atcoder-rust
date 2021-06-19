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
        h: usize,
        w: usize,
        s: [Chars; h],
    }

    let mut ans = "Yes";

    for i in 0..h {
        for si in &s[i] {
            // 文字が # の時に前後左右のいずれかが # なら塗り潰せる。
            if *si == '#' {
                

            }
        }
    }

    println!();
}
