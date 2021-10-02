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

    // 切られている箇所の番号をメモしとく配列
    let mut vec = vec![];
    vec.push(0);
    vec.push(l);
    vec.sort();

    // ci = 1 の時はきる場所をtrueにするだけ
    // ci = 2 の時は両端の最も近傍のtrueの位置を二分探索
    // 計算量はnlogn
    for _ in 0..q {
        input!(c: usize, x: usize);
        if c == 1 {
            vec.push(x);
            vec.sort();

        } else {
            let mut lower = 0;
            let mut upper = vec.len()-1;

            while upper - lower > 1 {
                let mid = (lower + upper) / 2;
                
                if x < vec[mid] {
                    upper = mid;
                } else {
                    lower = mid;
                }

            }

            let ans = vec[upper] - vec[lower];
            println!("{}", ans);
        }
    }
}
