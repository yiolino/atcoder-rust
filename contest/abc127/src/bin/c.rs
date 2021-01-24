use proconio::input;

fn main() {
    input! {
        _n: i32,
        m: i32,
        lr: [[i32; 2]; m],
    }

    let mut l = 0;
    let mut r = 1000_000;
    for p in lr {
        l = std::cmp::max(p[0], l);
        r = std::cmp::min(p[1], r);
    }

    let ans:i32;
    if r >= l {ans = r - l + 1;}
    else {ans = 0;}
    println!("{}", ans);
}
