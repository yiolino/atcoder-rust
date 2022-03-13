    use std::collections::BTreeSet;

use proconio::input;

fn main() {
    input!{
        n: usize,
        xy: [(usize, usize); n],
    }

    let mut set = BTreeSet::new();
    set.extend(xy);
    let mut ans = 0;
    
    for a in set.iter() {
        for b in set.iter() {
            if b.0 < a.0 && b.1 < a.1 && set.contains(&(a.0, b.1)) && set.contains(&(b.0, a.1)) {
                ans += 1;
            }
        }
    }
    println!("{}", ans);
}
