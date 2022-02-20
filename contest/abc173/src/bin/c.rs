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
        h: usize,
        w: usize,
        k: usize,
        c: [Chars; h]
    }

    let mut ans = 0;

    // bit全探索
    for bit_h in 0..1<<h {
        for bit_w in 0..1<<w {
            let mut cnt_black = 0;

            for hi in 0..h {
                if bit_h & 1 << hi == 0 {
                    for wi in 0..w {
                        if bit_w & 1 << wi == 0 {
                            if c[hi][wi] == '#' {
                                cnt_black += 1;
                            }
                        }
                    }
                }
            }

            if cnt_black == k {
                ans += 1;
            }
        }
    }    


    println!("{}", ans);
}
