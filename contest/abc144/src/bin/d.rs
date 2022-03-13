use proconio::input;

fn main() {
    input!{
        a: f64,
        b: f64,
        x: f64,
    }

    let ans = if a * a * b > x * 2. {
        (a * b * b / (2. * x)).atan() * 180.0 / std::f64::consts::PI
    } else {
        ((2. * b - 2. * x / (a * a)) / a).atan() * 180.0 / std::f64::consts::PI
    };


    println!("{}", ans);
}
