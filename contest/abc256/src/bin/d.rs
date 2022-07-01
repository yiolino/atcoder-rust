use proconio::input;

fn main() {
    input!{
        n: usize,
        mut lr: [(usize, usize); n],
    }

    lr.sort_by_key(|x| x.0);
    let mut vec = vec![];
    let mut interval: Option<(usize, usize)> = None;

    for lr in lr {
        if let Some(intv) = interval {
            if lr.0 <= intv.1 {
                interval = Some((intv.0, intv.1.max(lr.1)));
            } else {
                vec.push(intv);
                interval = Some((lr.0, lr.1))
            }
        } else {
            interval = Some(lr);
        }
    }

    if let Some(intv) = interval {
        vec.push(intv);
    }

    for v in vec {
        println!("{} {}", v.0, v.1)
    }
}
