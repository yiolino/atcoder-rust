use im_rc::HashSet;
use proconio::input;

fn main() {
    input!{
        mut s: String,
    }

    let mut sb = s.as_bytes().to_vec();
    let alphabet = (97..=122).collect::<Vec<u8>>();

    let mut set = HashSet::new();
    for s in sb.iter_mut() {
        set.insert(*s);
    }

    if set.len() >= 26 {
        println!("-1");
        return;
    }

    for i in (0..sb.len()).rev() {
        for a in &alphabet {
            if *a > sb[i] && !set.contains(a) {
                sb[i] = *a;
                println!("{}", String::from_utf8(sb).unwrap());
                return;
            }
        }
    }

    for a in &alphabet {
        if !set.contains(a) {
            sb.push(*a);
            break;
        }
    }

    println!("{}", String::from_utf8(sb).unwrap());
}
