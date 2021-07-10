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
        n: i64,
        q: usize,
        mut a: [i64; n],
        txy: [[i64; 3]; q],
    }

    let mut shift = 0;

    for i in 0..q {
        let t = txy[i][0];
        let x = txy[i][1];
        let y = txy[i][2];

        if t == 1 {
            let xi = (x - 1 - shift + n) % n;
            let yi = (y - 1 - shift + n) % n;
            let tmp_ax = a[xi as usize];
            let tmp_ay = a[yi as usize];
            a[xi as usize] = tmp_ay;
            a[yi as usize] = tmp_ax;
        
        } else if t == 2 {
            shift += 1;
            shift %= n;

        } else {
            let idx = (x - 1 - shift + n) % n;
            println!("{}", a[idx as usize]);
        }
    }

}
