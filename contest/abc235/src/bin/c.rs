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
        q: usize,
        a: [usize; n],
    }

    // メモ
    let mut memo: HashMap<usize, Vec<usize>> = HashMap::new();

    for (i, ai) in a.iter().enumerate() {
        if memo.contains_key(ai) {
            memo.get_mut(&*ai).unwrap().push(i);
        } else {
            memo.insert(*ai, vec![]);
            memo.get_mut(&*ai).unwrap().push(i);
        }
        //memo[*ai].push(i);
    }

    for _ in 0..q {
        input! {x: usize, k: usize};
        let key = memo.get(&x);
        match key {
            None => println!("-1"),
            Some(vec) => if vec.len() >= k {
                println!("{}", vec[k-1] + 1);
                } else {
                    println!("-1");
                }
            };
    }

}
