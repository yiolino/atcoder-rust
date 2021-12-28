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


// by sansen
fn main() {
    input! {
        n: usize,
        m: usize,
        s: [usize; n],
        t: [usize; m],
    }

    let mut ans = m; // 最低限必要な操作回数

    for &ti in t.iter() {
        // s0 と異なるものが出てくるまでスルー
        if ti == s[0] {
            continue;
        }

        // もし全てのsの要素が ti と異なるならいくら変形しても無理
        if s.iter().all(|s| *s != ti) {
            println!("-1");
            return;
        }

        let r = s.iter().position(|s| *s == ti).unwrap(); // 初めてsとtiが一致するidx
        let l = n - s.iter().rposition(|s| *s == ti).unwrap(); // 最後にsとtiが一致するidx

        ans += l.min(r) - 1;
        ans += t.windows(2).filter(|t| t[0] != t[1]).count(); // 前後で値が異なるときにカウント

        if t[0] != s[0] {
            ans += 1;
        }

        break; // 最後まで辿り着いたらiterationしない！
    }

    println!("{}", ans);
}



