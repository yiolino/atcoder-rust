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
        m: usize,
        a: [Chars; 2*n]
    }

    let mut cnt_vic = vec![0; 2*n];
    let mut order = vec![];
    for i in 0..2*n {
        order.push(i);
    }

    for i in 0..m {
        for j in 0..n {
            let te_1 = &a[order[2*j]][i];
            let te_2 = &a[order[2*j + 1]][i];

            if *te_1 == 'G' && *te_2 == 'C' {
                cnt_vic[order[2*j]] += 1;
            } else if *te_1 == 'C' && *te_2 == 'P' {
                cnt_vic[order[2*j]] += 1;
            } else if *te_1 == 'P' && *te_2 == 'G' {
                cnt_vic[order[2*j]] += 1;
            } else if *te_1 == 'C' && *te_2 == 'G' {
                cnt_vic[order[2*j + 1]] += 1;
            } else if *te_1 == 'P' && *te_2 == 'C' {
                cnt_vic[order[2*j + 1]] += 1;
            } else if *te_1 == 'G' && *te_2 == 'P' {
                cnt_vic[order[2*j + 1]] += 1;
            }
        }

        order = vec![];
        for i in (0..m+1).rev() {
            for j in 0..2*n {
                if cnt_vic[j] == i {
                    order.push(j);
                }
            }
        } 
    }

    for i in 0..2*n {
        println!("{}", order[i] + 1);
    }
}
