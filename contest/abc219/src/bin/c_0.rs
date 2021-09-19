#[allow(unused_imports)]
use proconio::{input, fastout, marker::Chars, marker::Usize1};
#[allow(unused_imports)]
use std::collections::{HashSet, HashMap, BTreeSet, VecDeque, BinaryHeap};
#[allow(unused_imports)]
use std::cmp::{max, min, Reverse};
#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use petgraph::unionfind::UnionFind;

#[fastout]
fn main() {
    input!{
        new_seq: Chars,
        n: usize,
    }

    let mut vec = vec![];
    let mut hozon = vec![];

    for idx in 0..n {
        let mut tmp_string = "".to_string();
        input!(word: Chars);
        hozon.push(word.iter().collect::<String>());

        for i in 0..word.len() {
            let wj = word[i];
            for j in 0..26 {
                let alp = new_seq[j];
                if alp == wj {
                    let add = (97 + j) as u8 as char;
                    tmp_string = tmp_string + &add.to_string();
                }
            }
        }

        vec.push((idx, tmp_string));
    }

    vec.sort_by_key(|(_, s)| s.clone());

    for (i, _) in vec {
        let ans = &hozon[i];
        println!("{}", ans);
    }
}
