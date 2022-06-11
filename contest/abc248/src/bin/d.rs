use proconio::input;
use superslice::Ext;

fn main() {
    input!{
        n: usize,
        a: [usize; n],
        q: usize,
    }

    // x毎にindexを保存しておき、二分探索を実施する。
    let mut index_mat = vec![vec![]; n+1];
    for (i, ai) in a.iter().enumerate() {
        index_mat[*ai].push(i);
    }

    for _ in 0..q {
        input! {l: usize, r: usize, x: usize};
        let lower = index_mat[x].lower_bound(&(l - 1));
        let upper = index_mat[x].lower_bound(&r);

        println!("{}", upper - lower);
    }
}
