use proconio::{input, fastout};
#[allow(unused_imports)]

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input!{
        A: usize,
        B: usize,
    }

    let mut counter = 0;
     for i in A..B+1 {
        let ic = i.to_string().chars().collect::<Vec<char>>();
        if ic[0] == ic[4] && ic[1] == ic[3] {counter += 1}
     }

    println!("{}", counter);
}
