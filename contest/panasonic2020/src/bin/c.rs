use proconio::input;

fn main() {
    input!{
        a: i64,
        b: i64,
        c: i64,
    }

    let x = c - a - b;
    let y = 4 * a * b;

    let ans = if x*x > y && x > 0 {
        "Yes"
    } else {
        "No"
    };

    println!("{}", ans);
}
