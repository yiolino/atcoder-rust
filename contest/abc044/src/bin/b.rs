use proconio::input;

fn main() {
    input!{
        w: String,
    }

    let mut count = vec![0; 26];

    for c in w.chars() {
        count[c as usize  - 'a' as usize] += 1;
    }

    for c in count {
        if c % 2 != 0 {
            println!("No");
            return;
        }
    }

    println!{"Yes"};
}
