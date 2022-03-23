use std::collections::BTreeMap;

use petgraph::unionfind::UnionFind;
use proconio::{input, marker::Usize1};

fn main() {
    input!{
        n: usize,
        m: usize,
        a: [i64; n],
        b: [i64; n],
    }

    let mut uf = UnionFind::new(n);

    for _ in 0..m {
        input! {c: Usize1, d: Usize1};
        uf.union(c, d);
    }

    let mut bmap = BTreeMap::new();
    for (i, par) in uf.into_labeling().into_iter().enumerate() {
        bmap.entry(par).or_insert(vec![]).push(i);
    }

    let mut ans = "Yes";
    for (_par, v) in bmap {
        let a_sum: i64 = v.iter().map(|vi| a[*vi]).sum();
        let b_sum: i64 = v.iter().map(|vi| b[*vi]).sum();

        if a_sum != b_sum {
            ans = "No";
            break;
        }
    }

    println!("{}", ans);
}
