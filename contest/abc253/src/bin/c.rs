use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        q: usize,
    }

    let mut bmap = BTreeMap::new();

    for _ in 0..q {
        input! {q: usize};
        match q {
            1 => {
                input! {x: usize};
                *bmap.entry(x).or_insert(0) += 1_usize;
            }
            2 => {
                input! {x: usize, c: usize};
                if let Some(a) = bmap.get_mut(&x) {
                    *a -= c.min(*a);

                    if bmap.get(&x).unwrap() == &0 {
                        bmap.remove(&x);
                    }
                }
            }
            3 => {
                let diff = bmap.iter().next_back().unwrap().0 - bmap.iter().next().unwrap().0;
                println!("{}", diff);
            }
            _ => unreachable!(),
        }
    }
}
