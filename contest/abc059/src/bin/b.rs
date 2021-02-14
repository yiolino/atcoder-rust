use proconio::{input,fastout};

#[fastout]
#[allow(non_snake_case)]
#[allow(unused_imports)]
fn main() {
    input!{
        A: String,
        B: String
    }

    let mut ans = "EQUAL".to_string();

    if A.len() > B.len() {
        ans = "GREATER".to_string();
    } else if A.len() < B.len() {
        ans = "LESS".to_string();
    } else {
        let vec_A: Vec<char> = A.chars().collect();
        let vec_B: Vec<char> = B.chars().collect();
        for i in 0..A.len() {
            if (vec_A[i] as i8) > (vec_B[i] as i8) {
                ans = "GREATER".to_string();
                break;
            } else if (vec_A[i] as i8) < (vec_B[i] as i8) {
                ans = "LESS".to_string();
                break;
            }
        }
    }

    println!("{}", ans);
}
