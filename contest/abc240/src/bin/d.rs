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

    let mut stack = vec![];
    let mut len = 0;

    for ai in a {
        let last = stack.last().cloned();
        match last {
            Some((l_a, num)) => {
                if l_a == ai {
                    let n = stack.len();
                    stack[n - 1] = (ai, num + 1);
                    len += 1;
                    if num + 1 == ai {
                        len -= ai;
                        stack.pop();  
                    } 
                } else {
                    stack.push((ai, 1_usize));
                    len += 1;
                }
            },
            None => {stack.push((ai, 1_usize)); len += 1},
        }

        println!("{}", len);
    }

}
