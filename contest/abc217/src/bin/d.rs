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
        l: usize,
        q: usize,
    }

    let mut btree = BTreeSet::new();
    btree.insert(0);
    btree.insert(l);

    for _ in 0..q {
        input!(c: usize, x: usize);
        if c == 1 {
            btree.insert(x);

        } else {
            let bfr = btree.before(x).unwrap();
            let aftr = btree.after(x).unwrap();

            println!("{}", aftr - bfr);
        }
    }
}



// BTreeSetのある値よりも1つ小さな値、もしくは1つ大きな値を返す。
trait Neighbors<T> {
    fn before(&self, x: T) -> Option<&T>;
    fn after(&self, x: T) -> Option<&T>;
}

impl<T: Ord> Neighbors<T> for BTreeSet<T> {
    fn before(&self, x: T) -> Option<&T> {
        let mut bfr = self.range((std::ops::Bound::Unbounded, std::ops::Bound::Excluded(x)));

        bfr.next_back()
    }

    fn after(&self, x: T) -> Option<&T> {
        let mut aftr = self.range((std::ops::Bound::Excluded(x), std::ops::Bound::Unbounded));

        aftr.next()
    }
}