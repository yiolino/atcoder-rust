use proconio::input;

fn main() {
    input!{
        n: f64,
    }

    let mut upper = std::f64::MAX;
    let mut lower = 0.0;

    while upper - lower > 0.0001 {
        let mid = (upper + lower) / 2.0;
        let tmp = f(mid);
        if tmp > n {
            upper = mid;
        } else {
            lower = mid;
        }
    }

    println!("{}", upper)
}


fn f(x: f64) -> f64 {
    x * x * x + x
}