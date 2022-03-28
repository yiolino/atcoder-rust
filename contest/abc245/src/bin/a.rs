use proconio::input;

fn main() {
    input!{
        a: usize,
        b: usize,
        c: usize,
        d: usize
    }

    let t = a * 60 + b;
    let a  = c * 60 + d + 1;

    let ans = if t < a {
        "Takahashi"
    } else {
        "Aoki"
    };

    println!("{}", ans);
}
