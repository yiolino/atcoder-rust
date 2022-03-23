use petgraph::unionfind::UnionFind;
use proconio::{input, marker::Usize1};

fn main() {
    input!{
        n: usize,
        m: usize,   
    }

    let mut uf = UnionFind::new(n);

    for _ in 0..m {
        input! {a: Usize1, b: Usize1};
        uf.union(a, b);
    }

    let mut vec = vec![0_usize; n];
    for par in uf.into_labeling() {
        vec[par] += 1;
    }

    let ans = vec.iter().max().unwrap();

    println!("{}", ans);
}
