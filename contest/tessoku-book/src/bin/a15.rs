use itertools::Itertools;
use proconio::input;
use superslice::Ext;

fn main() {
    input!{
        n: usize,
        a: [usize; n],
    }

    let mut z = a.clone();
    z.sort();
    z.dedup();

    let mut b = vec![];
    
    for ai in a {
        let bi = z.lower_bound(&ai);
        b.push(bi + 1);
    }

    let ans = b.into_iter().join(" ");
    println!("{}", ans)
}
