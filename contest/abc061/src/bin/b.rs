#[allow(unused_imports)]
use proconio::{input,fastout, marker::Chars};

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input!{
        N: usize, 
        M: usize,
        ab: [usize; 2*M],
    }

    let mut counter_vec = vec![0; N];
    for i in ab {
        counter_vec[(i - 1)as usize] += 1;  
    }

    for i in counter_vec {
        println!("{}", i);
    }
}
