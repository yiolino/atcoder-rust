use petgraph::unionfind::UnionFind;
use proconio::{input, marker::Usize1};

fn main() {
    input!{
        n: usize,
        m: usize,
    }

    // union find
    let mut deg = vec![0; n];
    let mut uf = UnionFind::new(n);
    for _ in 0..m {
        input! {a: Usize1, b: Usize1};
        if uf.equiv(a, b) {
            println!("No");
            return;
        }
        deg[a] += 1;
        deg[b] += 1;

        uf.union(a, b);
    }

    for i in 0..n {
        if deg[i] > 2 {
            println!("No");
            return;
        }
    }

    println!("Yes");
    return;
}
