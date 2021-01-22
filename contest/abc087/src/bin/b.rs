use proconio::input;

fn main() {
    input!{
        a: i32,
        b: i32,
        c: i32,
        x: i32,
    }

    let mut counter = 0;

    for i in 0..a+1 {
        for j in 0..b+1 {
            for k in 0..c+1 {
                if 500 * i + 100 * j + 50 * k == x {counter += 1;}
            }
        }
    }

    println!("{}", counter);
}
