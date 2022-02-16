use num_integer::{gcd, lcm};
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
        n: usize,
        m: usize,
        s: Chars,
        t: Chars,
    }

    let gcd = gcd(n, m);

    let mut is_match_all = true;

    let mut i = 0;
    while i * n / gcd < s.len() && i * m / gcd < t.len() {
        if s[i * n / gcd] != t[i * m / gcd] {
            is_match_all = false;
            break;
        }

        i += 1;
    }

    if is_match_all {
        println!("{}", lcm(n, m));
    } else {
        println!("{}", -1);
    }
}
