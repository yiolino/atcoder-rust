use proconio::input;

fn main() {
    input!{
        t: usize,
        n: usize,
    }

    let mut cum = vec![0; t + 2];

    for _ in 0..n {
        input! {l: usize, r: usize};
        cum[l] += 1;
        cum[r] -= 1;
    }

    for i in 1..t+2 {
        cum[i] = cum[i] + cum[i - 1];
    }

    for i in 0..t {
        println!("{}", cum[i])
    }
}
