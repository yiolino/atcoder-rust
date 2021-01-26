use proconio::input;

fn main() {
    input!{
        n:i64,
        k:i64,
        q:i64,
        array_a:[i64; q],
    }

    let mut point = vec![k-q; n as usize];


    for a in array_a {
        point[(a-1) as usize] += 1;
    }

    for p in point {
        if p > 0 {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
