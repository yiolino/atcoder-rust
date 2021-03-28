#[allow(unused_imports)]
use proconio::{input, fastout, marker::Chars};
#[allow(unused_imports)]
use std::collections::{HashSet, HashMap, BTreeSet};
#[allow(unused_imports)]
use std::cmp::{max, min};

#[allow(non_snake_case)]
fn main() {
    input!{
        T: usize,
    }

    for _i in 0..T { 
        input!{
            N: usize,
        }

        if N % 4 == 0 {
            println!{"{}", "Even"};
        } else if N % 4 == 1 {
            println!{"{}", "Odd"};
        } else if N % 4 == 3 {
            println!{"{}", "Odd"};
        } else {
            println!("{}", "Same");
        }
    }

}
