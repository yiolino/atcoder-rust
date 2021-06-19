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
        n: usize,
        m: usize,
        mut a: [usize; m],
    }

    if m == 0 {
        println!("{}", 1);
        return;
    }

    a.sort();

    let mut min = std::usize::MAX;

    let mut map = std::collections::HashMap::new();


    for i in 0..m-1 {
        let diff = a[i+1] - a[i] - 1;
        *map.entry(diff).or_insert(0) += 1;

        if diff != 0 {
            min = min.min(diff);
        }
    }

    // 両端
    if a[0] != 1 {
        let diff = a[0] - 1;
        *map.entry(diff).or_insert(0) += 1;

        if diff != 0 {
            min = min.min(diff);
        }
    }
    if a[m-1] != n {
        let diff = n - a[m-1];
        *map.entry(diff).or_insert(0) += 1;

        if diff != 0 {
            min = min.min(diff);
        }
    }

    let mut cnt = 0;
    for (k, v) in map {
        if k == 0 {
            continue;
        }

        let mut sho = k / min;
        let amari = k % min;

        if amari != 0 {
            sho += 1;
        }

        cnt += sho * v;
    }

    println!("{}", cnt);
}
