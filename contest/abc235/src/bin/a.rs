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
        vec: Chars,
    }

    let abc: i32 = vec.iter().collect::<String>().parse().unwrap();

    let mut tmp_bca = vec![];
    tmp_bca.push(vec[1]);
    tmp_bca.push(vec[2]);
    tmp_bca.push(vec[0]);
    let bca: i32 = tmp_bca.iter().collect::<String>().parse().unwrap();

    let mut tmp_cab = vec![];
    tmp_cab.push(vec[2]);
    tmp_cab.push(vec[0]);
    tmp_cab.push(vec[1]);
    let cab: i32 = tmp_cab.iter().collect::<String>().parse().unwrap();


    println!("{}", abc + bca + cab);
}
