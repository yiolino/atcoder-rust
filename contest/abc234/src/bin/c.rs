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
        mut k: usize,
    }

    // 2進数表記と対応していることを利用する。

    let mut bit_vec = vec![];

    while k > 0 {
        // 2の余りを見れば最小の桁が0か1かわかる
        if k % 2 == 1 {
            bit_vec.push('2');  // この問題では2を入れる
        } else {
            bit_vec.push('0');
        }

        // 2で割るとシフトすることとと対応
        // 次の桁を見る
        k /= 2;
    }

    bit_vec.reverse();
    let ans = bit_vec.iter().collect::<String>();

    println!("{}", ans);

    println!("{:08b} {:08b}", 1_u8, !1_u8);
}

