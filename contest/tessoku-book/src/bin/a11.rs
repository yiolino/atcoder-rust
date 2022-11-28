use proconio::input;

fn main() {
    input!{
        n: usize,
        x: usize,
        mut a: [usize; n]
    }

    a.sort();

    let mut lower = 0;
    let mut upper = n - 1;
    while upper - lower > 1 {
        let mid = (upper + lower) / 2;
        if a[mid] >= x {
            upper = mid;
        } else {
            lower = mid;
        }
    }

    println!("{}", upper + 1);
}
