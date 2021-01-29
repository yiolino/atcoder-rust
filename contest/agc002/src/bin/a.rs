use proconio::input;

fn main() {
    input! {
        a: i128,
        b: i128,
    }

    if a <= 0 && 0 <= b {
        println!("Zero");
    } else if a == b {
        println!("Positeve");
    } else if a > 0 {
        println!("Positive");
    } else if b < 0 {
        if (b - a) % 2 == 0 {println!("Negative");}
        else {println!("Positive");}
    }
}
