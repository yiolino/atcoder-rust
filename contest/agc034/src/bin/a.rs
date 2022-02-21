#[allow(unused_imports)]
use proconio::{input, fastout, marker::Chars, marker::Usize1, marker::Bytes};
#[allow(unused_imports)]
use std::collections::{HashSet, HashMap, BTreeSet, VecDeque, BinaryHeap};
#[allow(unused_imports)]
use std::cmp::{max, min, Reverse};
#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use petgraph::unionfind::UnionFind;
#[allow(unused_imports)]
use superslice::Ext;

fn main() {
    input!{
        _n: usize,
        a: usize,
        b: usize,
        c: usize,
        d: usize,
        mut s: Chars,
    }

    // 1-indexにしとく
    s.insert(0, '?');

    // c < d と c > d に場合分けして考える。

    if c < d {
        // b -> d の間で連続した ## がないかチェック
        let mut m = 0;
        for i in b..d {
            if s[i] == '#' {
                m += 1;
                if m == 2 {
                    println!("No");
                    return;
                }
            } else {
                m = 0;
            }
        }

        // a -> c の間で連続した ## がないかチェック
        let mut m = 0;
        for i in a..c {
            if s[i] == '#' {
                m += 1;
                if m == 2 {
                    println!("No");
                    return;
                }
            } else {
                m = 0;
            }
        }

    } else {
        // c > d の場合は1工夫必要
        let mut m = 0;
        let mut l = 0;  // .が連続するか
        let mut is_l_ser3 = false;
        for i in b-1..=d+1 {
            if s[i] == '#' {
                l = 0;
                m += 1;
                if m == 2 {
                    println!("No");
                    return;
                }
            } else {
                m = 0;
                l += 1;
                if l == 3 {
                    is_l_ser3 = true;
                }
            }
        }

        // a -> c の間で連続した ## がないかチェック
        let mut m = 0;
        for i in a..c {
            if s[i] == '#' {
                m += 1;
                if m == 2 {
                    println!("No");
                    return;
                }
            } else {
                m = 0;
            }
        }

        if !is_l_ser3 {
            println!("No");
            return;
        }
    }



    println!("Yes");
}
