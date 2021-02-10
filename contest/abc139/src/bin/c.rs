use proconio::{input,fastout};

#[fastout]
#[allow(non_snake_case)]
#[allow(unused_imports)]
fn main() {
    input!{
        N: i64,
        H: [i64; N],
    }

    let mut idx = 0;
    let mut counter = 0;
    let mut max = 0;

    while idx <= N - 2 {
        let left = H[idx as usize];
        let right = H[(idx + 1) as usize];
        if left >= right {
            counter += 1;
        } else {
            counter = 0;
        }
        max = std::cmp::max(max, counter);
        idx += 1;
    }

    println!("{}", max);
}
