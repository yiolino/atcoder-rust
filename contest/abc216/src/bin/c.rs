#[allow(unused_imports)]
use proconio::{input, fastout, marker::Chars, marker::Usize1};
#[allow(unused_imports)]
use std::collections::{HashSet, HashMap, BTreeSet, VecDeque, BinaryHeap};
#[allow(unused_imports)]
use std::cmp::{max, min, Reverse};
#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use petgraph::unionfind::UnionFind;

#[fastout]
fn main() {
    input!{
        mut n: usize,
    }

    let mut vec = vec![];

    while n != 0 {
        if n % 2 == 0 {
            n /= 2;
            vec.push('B');
        } else {
            n -= 1;
            vec.push('A');
        }
    }

    vec.reverse();

    let ans = vec.into_iter().collect::<String>();

    println!("{}", ans);
}
