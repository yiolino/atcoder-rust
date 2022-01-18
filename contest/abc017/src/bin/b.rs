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
        x: Chars,
    }

    
    if x.len() == 0 {
        println!("YES");
        return;
    }

    let mut vec = vec![];

    for c in x {
        match c {
            'o' => (),
            'k' => (),
            'u' => (),
            _ => vec.push(c),
        };
    }

    if vec.len() % 2 != 0 {
        println!("NO");
        return;
    } else if vec.len() == 0 {
        println!("YES");
        return;
    }

    let is_choku = vec.windows(2)
                    .all(|ch| ch[0] == 'c' && ch[1] == 'h');

    if is_choku {
        println!("YES");
    } else {
        println!("NO");
    }
}
