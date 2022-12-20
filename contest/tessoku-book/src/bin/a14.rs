use proconio::input;
use superslice::Ext;

fn main() {
    input!{
        n: usize,
        k: usize,
        a: [usize; n],
        b: [usize; n],
        c: [usize; n],
        d: [usize; n],
    }

    let mut p = vec![0; n * n];
    let mut q = vec![0; n * n];

    for (i, ai) in a.iter().enumerate() {
        for (j, bj) in b.iter().enumerate() {
            p[i * n + j] = ai + bj;
        }
    }

    for (i, ci) in c.iter().enumerate() {
        for (j, dj) in d.iter().enumerate() {
            q[i * n + j] = ci + dj;
        }
    }

    q.sort();

    for pi in p {
        let idx = q.lower_bound(&(k - pi));
        if idx < n *n && q[idx] == k - pi {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
