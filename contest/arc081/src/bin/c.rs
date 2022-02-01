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
        a: [usize; n],
    }

    let mut map = HashMap::new();

    for ai in a {
        *map.entry(ai).or_insert(0) += 1;
    }

    let mut map:Vec<(usize, usize)> = map.into_iter().collect();
    map.sort_by_key(|(k, _v)| std::cmp::Reverse(*k));

    
    let mut cnt = 0;
    let mut ans = 0;
    let mut cand = vec![];
    for (k, v) in &map {
        if *v >= 4 {
            ans = ans.max(k * k);
            cnt += 1;
            cand.push(*k);
        }
        else if *v >= 2 {
            cnt += 1;
            cand.push(*k);
        }

        if cnt == 2 {
            break;
        }
    } 

    if cnt == 2 {
        println!("{}", ans.max(cand[0] * cand[1]));
        return;
    }

    println!("0");
}
