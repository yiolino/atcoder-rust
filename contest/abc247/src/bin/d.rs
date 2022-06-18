use std::collections::VecDeque;

use proconio::input;

fn main() {
    input!{
        q: usize,
    }

    let mut deque = VecDeque::new();

    for _ in 0..q {
        input! {a: usize};
        if a == 1 {
            input! {x: usize, c: usize};
            deque.push_back((x, c));
        } else {
            input! {mut c: usize};
            let mut ans = 0;
            while c > 0 {
                let ref_front = deque.get_mut(0).unwrap();
                if ref_front.1 >= c {
                    ans += ref_front.0 * c;
                    ref_front.1 -= c;
                    c = 0;
                } else {
                    ans += ref_front.0 * ref_front.1;
                    c -= ref_front.1;
                    deque.pop_front();
                }
            }

            println!("{}", ans)
        }
    }
}
