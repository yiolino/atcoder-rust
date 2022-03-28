use im_rc::HashSet;
use proconio::input;

fn main() {
    input!{
        n: usize,
        a: [usize; n]
    }

    let mut set = HashSet::new();

    for a in a {
        set.insert(a);
    }

    for i in 0..=2001 {
        if !set.contains(&i) {
            println!("{}", i);
            return;
        }
    }
}
