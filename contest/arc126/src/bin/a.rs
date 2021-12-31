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
        t: usize,
    }

    for _ in 0..t {
        input! {mut n1: usize,
                mut n3: usize,
                mut n2: usize,
            };

        // 長さ3の棒は必ず偶数本使用する
        n3 /= 2;

        let mut ans = 0;

        if n2 >= 1 && n3 >= 1 {
            let k = min(n2, n3);
            ans += k;
            n2 -= k;
            n3 -= k;
        }

        if n3 >= 1 && n1 >= 2 {
            let k = min(n3, n1 / 2);
            ans += k;
            n1 -= 2 * k;
        }
            
        if n2 >= 2 && n1 >= 1 {
            let k = min(n2 / 2, n1);
            ans += k;
            n2 -= 2 * k;
            n1 -= k;
        }

        
        if n2 >= 1 && n1 >= 3 {
            let k = min(n2, n1 / 3);
            ans += k;
            n1 -= 3 * k;
        }

        if n1 >= 5 {
            ans += n1 / 5;
        }

     
        println!("{}", ans);
    }
}
