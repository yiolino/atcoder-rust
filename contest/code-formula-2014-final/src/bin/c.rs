use std::collections::BTreeSet;
use regex::Regex;

fn main() {
    let mut s = String::new();
    std::io::stdin().read_line(&mut s).unwrap();

    let mut bset = BTreeSet::new();
    let re = Regex::new(r"@+([a-z]*)").unwrap();  // get(0) で全体を、get1で（）内で一致した部分をgetできる
    for it in s.split_whitespace() {
        for cap in re.captures_iter(it) {
            bset.insert(cap.get(1).unwrap().as_str());
        }
    }

    for b in bset {
        if b.is_empty() {
            continue;
        }
        println!("{}", b);
    }
}

