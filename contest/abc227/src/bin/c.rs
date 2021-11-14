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

#[fastout]
fn main() {
    input!{
        n: usize,
    }

    let mut ans = 0;

    let mut a = 1;

    while a*a*a <= n {
        let mut b = a;
        while b*b <= n / a {
            ans += n / a / b - b + 1;
            b += 1;
        }
        a+= 1;
    }
    
    println!("{}", ans);

}
