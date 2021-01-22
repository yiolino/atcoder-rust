use proconio::input;

fn main() {
    input!{
        a: u32,
        _b: u32,
        s: String,
    }

    let vec:Vec<bool> = s.chars().map(|x| x == '-').collect();
    let ans:&str;
    
    if vec[a as usize] && vec.iter().filter(|&bl| *bl == true).count() == 1 {ans = "Yes";}
    else {ans = "No";}

    println!("{}", ans);
}
