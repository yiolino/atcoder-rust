use proconio::input;

fn main() {
    input!{
        x: i64,
        y: i64,
    }

    let ans = if (x < 0 && y <= 0) || (x >= 0 && y > 0) {
        if x <= y {
            y - x
        } else {
            x - y + 2
        }
    } else {
        (x + y).abs() + 1
    };

    println!("{}", ans);
}
