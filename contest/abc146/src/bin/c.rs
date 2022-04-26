use proconio::input;

fn main() {
    input!{
        a: usize,
        b: usize,
        x: usize,
    }

    let mut lower = 0;
    let mut upper = 1_000_000_000 + 1;

    while upper - lower > 1 {
        let mid = (upper + lower) / 2;
        let p = calc_price(a, b, mid);

        if p <= x {
            lower = mid;
        } else {
            upper = mid;
        }
    }

    println!("{}", lower);
}

fn calc_price(a: usize, b: usize, n: usize) -> usize {
    let d = d(n);
    let res = a * n + b * d;

    res
}

fn d(n: usize) -> usize {
    let mut m = n;
    let mut res = 0;
    while m > 0 {
        m /= 10;
        res += 1;
    }

    res
}