use proconio::input;

fn main() {
    input!{
        n: usize,
        mut a: [f64; n],
    }

    a.sort_by(|a, b| a.partial_cmp(b).unwrap());

    // 三分探索で解く
    let mut lower = 0.0;
    let mut upper = **&a.last().unwrap();

    while upper - lower > 1e-7 {
        let t1 = (2. * lower + upper) / 3.;
        let t2 = (lower + 2. * upper) / 3.;

        if f(n, t1, &a) <= f(n, t2, &a) {
            upper = t2;
        } else {
            lower = t1;
        }
    }

    let ans = f(n, lower, &a) / n as f64;

    println!("{}", ans);
}


fn f(n:usize, x: f64, a: &Vec<f64>) -> f64 {
    let mut res = 0.;

    for a in a {
        res += a - a.min(2. * x);
    }

    res += n as f64 * x;
    res
}