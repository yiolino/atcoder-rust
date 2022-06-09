use proconio::{input, marker::Chars};

fn main() {
    input!{
        h: usize,
        w: usize,
        a: [Chars; h],
    }

    let mut num_sharp = 0;
    for ai in a {
        for c in ai {
            if c == '#' {
                num_sharp += 1;
            }
        }
    }

    let ans = if num_sharp == h + w - 1{
        "Possible"
    } else {
        "Impossible"
    };
    println!("{}", ans);
}
