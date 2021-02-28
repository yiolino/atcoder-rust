#[allow(unused_imports)]
use proconio::{input, fastout, marker::Chars};
#[allow(unused_imports)]
use std::collections::HashMap;

#[fastout]
#[allow(non_snake_case)]
fn main() {
    input!{
        k: usize, s: Chars, t: Chars,
    }

    let s = &s[..4]
            .into_iter()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect::<Vec<usize>>();
    
    let t = &t[..4]
            .into_iter()
            .map(|c| c.to_digit(10).unwrap() as usize)
            .collect::<Vec<usize>>();
    
    let mut remains = vec![k; 10];
    let mut s_points = (0..10).collect::<Vec<usize>>();
    let mut t_points = (0..10).collect::<Vec<usize>>();

    for i in 0..4 {
        remains[t[i]] -= 1;
        remains[s[i]] -= 1;
        s_points[s[i]] *= 10;
        t_points[t[i]] *= 10;
    }

    let s_sum = s_points.iter().sum::<usize>();
    let t_sum = t_points.iter().sum::<usize>();

    let mut count = 0;
    for i in 1..10 {
        if remains[i] == 0 {
            continue;
        }
        remains[i] -= 1;
        for j in 1..10 {
            if remains[j] == 0 {
                continue;
            }

            if s_sum + 9 * s_points[i] > t_sum + 9 * t_points[j] {
                count += remains[j] * (remains[i] + 1);
            }
        }
        remains[i] += 1;
    }

    let d =(9.0 * k as f64 - 8.0) * (9.0 * k as f64 - 9.0);
    println!("{}", count as f64 / d);
}
