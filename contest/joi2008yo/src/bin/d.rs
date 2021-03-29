#[allow(unused_imports)]
use proconio::{input, fastout, marker::Chars};
#[allow(unused_imports)]
use std::collections::{HashSet, HashMap, BTreeSet};
#[allow(unused_imports)]
use std::cmp::{max, min};

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input!{
        m: usize,
        star: [(i64, i64); m],
        n: usize,
        pic: [(i64, i64); n],
    }

    let mut ans_cnd = vec![];

    for i in 0..n {
        let x = pic[i].0 - star[0].0;
        let y = pic[i].1 - star[0].1;

        ans_cnd.push((x, y));
    }

    let mut set = HashSet::new();

    for p in &pic {
        set.insert(p);
    }

    let mut ans = (0, 0);

    for (x, y) in &ans_cnd {
        if star.iter().all(|(xs, ys)| set.contains(&(x+xs, y+ys))) {
            ans = (*x, *y);
        }

    }

    println!("{} {}", ans.0, ans.1);
}
