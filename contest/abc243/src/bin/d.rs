use proconio::{input, marker::Chars};

fn main() {
    input!{
        _n: usize,
        x: usize,
        s: Chars
    }

    let mut d = format!("{:b}", x);
    for si in s {
        match si {
            'U' => {d.pop();},
            'L' => d.push('0'),
            'R' => d.push('1'),
            _ => unreachable!(),
        }
    } 

    println!("{}", i64::from_str_radix(&d, 2).unwrap());
}
