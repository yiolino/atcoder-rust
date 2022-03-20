use proconio::input;

fn main() {
    input!{
        n: usize,
        mut a: [f64; n],
    }

    a.sort_by(|a, b| a.partial_cmp(b).unwrap());

    let mut cum = vec![0.0; n + 1];
    for i in 0..n {
        cum[i + 1] = cum[i] + a[i];
    }

    let mut min_f = std::f64::MAX;

    for i in 0..n {
        let x = a[i] / 2.0;
        let f = n as f64 * x - (cum[i + 1] + 2.0 * x * (n - i - 1) as f64);

        min_f = min_f.min(f);
    }

    let ans = (min_f + cum[n]) / n as f64;

    println!("{}", ans);
}
