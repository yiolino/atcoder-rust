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
        M: usize,
        xy: [(usize, usize); M],
    }

    let mut relation = vec![vec![false; N]; N];

    for (x, y) in xy {
        relation[x-1][y-1] = true;
        relation[y-1][x-1] = true;
    }

    let mut ans = 0;

    for bit in 0..1 << N {
        let mut is_ok = true;
        let num = (bit as u64).count_ones();

        for i in 1..N {
            for j in 0..i {
                if (bit >> i & 1 > 0) && (bit >> j & 1 > 0) {
                    if !relation[i][j] {
                        is_ok = false;
                    }
                }   
            }
        }

        if is_ok && num > ans {
            ans = num;
        }
    }

    println!("{}", ans);
}
