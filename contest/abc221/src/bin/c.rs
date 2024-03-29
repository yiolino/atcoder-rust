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
        n: Bytes,
    }

    let mut vec = vec![];

    for ni in n {
        vec.push(ni - 48);
    }

    vec.sort();

    let mut ans = 0;

    loop {
        for i in 1..vec.len() {
            let mut a = vec![];
            let mut b = vec![];

            for j in 0..i {
                a.push(vec[j]);
            }
            for j in i..vec.len() {
                b.push(vec[j]);
            }

            if a[0] == 0 || b[0] == 0 {
                continue;
            }
    
            let aa: usize = a.into_iter()
                            .map(|x| (x+48) as char)
                            .collect::<String>()
                            .parse().unwrap();
    
            let bb: usize = b.into_iter()
                            .map(|x| (x+48) as char)
                            .collect::<String>()
                            .parse().unwrap();
    
            ans = ans.max(aa * bb);
        }



        if !vec.next_permutation() {
            break;
        }
    }

    println!("{}", ans);
}

