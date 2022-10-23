use proconio::input;

fn main() {
    input!{
        d: usize,
        n: usize,
    }

    let mut cum = vec![0; d + 2];

    for _ in 0..n {
        input! {l: usize, r: usize};
        cum[l] += 1;
        cum[r + 1] -= 1;
    }

    for i in 1..(d + 2) {
        cum[i] = cum[i] + cum[i - 1];
    }

    for i in 1..d+1 {
        println!("{}", cum[i])
    }
}
