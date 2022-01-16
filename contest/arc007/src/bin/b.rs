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
        mut d: [usize; m],
    }

    // n ... CDケースの個数
    // m ... 今日聞いたCDケースの枚数

    let mut ans = (1..=n).collect::<Vec<usize>>();

    let mut playing = 0;
    for di in d {
        if di != 0 {
            if di == playing {
                continue;
            }

            let pos = ans.iter()
                            .enumerate()
                            .filter(|(_i, disk)| **disk == di)
                            .map(|(i, _disk)| i)
                            .collect::<Vec<usize>>()[0];

            std::mem::swap(&mut playing, &mut ans[pos]);

        } else {
            if di == playing {
                continue;
            }

            let zero_pos = ans.iter()
                            .enumerate()
                            .filter(|(_i, disk)| **disk == 0)
                            .map(|(i, _disk)| i)
                            .collect::<Vec<usize>>()[0];
            
            std::mem::swap(&mut playing, &mut ans[zero_pos]);
        }
        
    }

    for a in ans {
        println!("{}", a)
    }
}
