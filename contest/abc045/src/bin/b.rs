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
        a: Chars,
        b: Chars,
        c: Chars,
    }

    let mut a = VecDeque::from(a);
    let mut b = VecDeque::from(b);
    let mut c  = VecDeque::from(c);

    let mut now = a.pop_front();
    let mut ans = '0';
    loop {
        match now {
            Some(v) => match v {
                'a' =>  {now = a.pop_front(); ans = 'A'},
                'b' => {now = b.pop_front(); ans = 'B'},
                'c' => {now = c.pop_front(); ans = 'C'},
                _ => (),
            },
            None => break,
        }
    }

    println!("{}", ans);
}
