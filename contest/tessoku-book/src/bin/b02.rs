use proconio::input;

fn main() {
    input!{
        a: usize,
        b: usize
    }

    for i in a..=b {
        if 100 % i == 0 {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
