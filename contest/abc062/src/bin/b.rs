#[allow(unused_imports)]
use proconio::{input,fastout};

#[allow(non_snake_case)]
fn main() {
    input!{
        H: usize,
        W: usize,
        pic: [String; H],
    }

    println!("{}", "#".repeat(W + 2 as usize));

    for i in 0..H {
        print!("#");
        for j in pic[i].as_str().chars() {
            print!{"{}", j};
        }
        println!("#");
    }

    println!("{}", "#".repeat(W + 2 as usize));
}
