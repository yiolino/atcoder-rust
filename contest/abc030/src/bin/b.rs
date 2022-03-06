use proconio::input;

fn main() {
    input!{
        n: usize,
        m: f64
    }

    let b = m * 6.0;
    let a = (n % 12) as f64 * 30.0 + m / 2.0;
    

    println!("{}", (a - b).abs().min(360.0 - (b - a).abs()));
}
