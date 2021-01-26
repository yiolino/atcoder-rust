use proconio::input;

fn maxtime(x: &i64) -> i64 {
    (x + 9) / 10 * 10
}

fn remtime(x: &i64) -> i64 {
    maxtime(x) - x
}

fn main() {
    input!{
        meal: [i64; 5],
    }

    let mut sum = 0;
    
    for m in &meal {
        sum += maxtime(m);
    }

    sum -= meal.iter().map(|x| remtime(x)).max().unwrap();

    println!("{}", sum);
}
