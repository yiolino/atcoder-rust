use proconio::input;

fn main() {
    input!{
        n: usize,
        k: usize,
        a: [usize; n],   
    }

    let mut upper = 1000_000_000;
    let mut lower = 0;

    while upper - lower > 1 {
        let mid = (upper + lower) / 2;
        if check(mid, k, &a) {
            upper = mid;
        } else {
            lower = mid;
        }
    }

    println!("{}", upper);
}

// k枚印刷するのにx秒以内ならtrue
fn check(x: usize, k: usize, a: &Vec<usize>) -> bool {
    let mut p = 0;
    for ai in a {
        p += x / ai;
    }

    if p >= k {
        true
    } else {
        false
    }
}