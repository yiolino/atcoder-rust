use proconio::input;

fn main() {
    input!{
        x: usize,
    }

    let mut i = 1;
    while i * (i + 1) / 2 <= x {
        let t = i * (i + 1) / 2;
        if t >= x {
            break;
        }

        i += 1;
    }

    println!("{}", i);
}
