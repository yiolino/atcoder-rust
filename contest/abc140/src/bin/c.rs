#[allow(unused_imports)]
use proconio::{input,fastout, marker::Chars};

#[fastout]
#[allow(non_snake_case)]
#[allow(unused_imports)]
fn main() {
    input!{
        N: usize,
        B: [usize; N - 1],
    }

    let mut A = vec![0; N];

    A[0] = B[0];
    // A[1] = B[0];

    for i in 0..N - 2 {
        A[i + 1] = std::cmp::min(B[i], B[i + 1]);
        // A[i + 1] = std::cmp::max(A[i + 1], B[i]);
    }
    A[N - 1] = B[N - 2];

    let mut ans = 0;
    for i in 0..N {
        ans += A[i];
    }
  
    println!("{}", ans);
}
