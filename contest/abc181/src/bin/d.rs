    #[allow(unused_imports)]
use proconio::{input, fastout, marker::Chars, marker::Usize1, marker::Bytes};
#[allow(unused_imports)]
use std::collections::{HashSet, HashMap, BTreeSet, VecDeque, BinaryHeap};
#[allow(unused_imports)]
use std::cmp::{max, min, Reverse};
use std::mem::swap;
#[allow(unused_imports)]
use itertools::Itertools;
#[allow(unused_imports)]
use petgraph::unionfind::UnionFind;
#[allow(unused_imports)]
use superslice::Ext;

fn main() {
    input!{
        mut s: Chars,
    }

    if s.len() == 1 {
        if s[0] == '8' {
            println!("Yes");
            return;
        }
    }

    if s.len() == 2 {
        let i = s.iter().collect::<String>()
                        .parse::<usize>()
                        .unwrap();
        if i % 8 == 0 {
            println!("Yes");
            return;
        }

        s.swap(0, 1);
        let i = s.iter().collect::<String>()
                        .parse::<usize>()
                        .unwrap();
        if i % 8 == 0 {
            println!("Yes");
            return;
        }

    }

    let mut vec = vec![0; 10];

    for si in s {
        let i = si.to_digit(10).unwrap();
        vec[i as usize] += 1;
    }

    let mut cnt = 0;
    let mut ans = "No";
    while cnt < 1000 {
        let mut sc = cnt.to_string().chars().collect::<Vec<char>>();
        if sc.len() == 1 {
            sc.insert(0, '0');
            sc.insert(0, '0');
        } else if sc.len() == 2 {
            sc.insert(0, '0');
        }

        let mut tmp_vec = vec![0; 10];
        for c in sc {
            let i = c.to_digit(10).unwrap();
            tmp_vec[i as usize] += 1;
        }

        let is_ok = (0..10).all(|i| vec[i] >= tmp_vec[i]);

        if is_ok {
            ans = "Yes";
            break;
        }

        cnt += 8;
    }


    println!("{}", ans);
}
