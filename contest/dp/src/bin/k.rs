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
        k: usize,
        a: [usize; n],
    }

    // dp[i] := 残りi個の時、先手が勝つならtrue、先手が引けないならfalse
    let mut dp = vec![false; k+1];
    
    for i in 1..=k {
        for j in 0..n {
            let jmp = i as i32 - a[j] as i32;
            if jmp >= 0 && dp[jmp as usize] == false {
                dp[i] = true;
            }
        }
    }

    let ans = if dp[k] {
        "First"
    } else {
        "Second"
    };
    
    println!("{}", ans);
}
