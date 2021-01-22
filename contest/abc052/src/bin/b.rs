use proconio::input;

fn main() {
    input!{
        _n: i32,
        s: String,
    }

    let mut x = 0;
    let mut max = 0;
    for si in s.chars() {
        if si == 'I' {x += 1;}
        else {x -= 1}

        max = std::cmp::max(x, max);
    }

    println!("{}", max)
}


