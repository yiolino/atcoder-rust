use proconio::input;

fn main() {
    input!{
        n: usize,
        q: usize,
        a: [usize; n],
    }

    let mut cum = vec![0; n + 1];
    for (i, ai) in a.iter().enumerate() {
        cum[i + 1] = cum[i] + ai;
    }

    for _ in 0..q {
        input! {l: usize, q: usize};
        
        let ans = cum[q] - cum[l - 1];
        println!("{}", ans);
    }
}
