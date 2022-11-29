use proconio::input;
use superslice::Ext;

fn main() {
    input!{
        n: usize,
        mut a: [usize; n],
        q: usize,
    }

    a.sort();

    for _ in 0..q {
        input! {x: usize};
        let ans = a.lower_bound(&x);

        println!("{}", ans)
    }
}
