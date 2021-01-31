use proconio::{derive_readable, fastout, input};

#[derive_readable]
struct Shop {
    price: u64,
    n_stock: u64,
}

#[fastout]
fn main() {
    input! {
        n: usize,
        m: u64,
        mut shops: [Shop; n]
    }

    shops.sort_by_key(|s| s.price);

    let mut cost = 0;
    let mut n_need = m;

    for shop in shops {
        if n_need > shop.n_stock {
            cost += shop.price * shop.n_stock;
            n_need -= shop.n_stock;
        } else {
            cost += n_need * shop.price;
            break;
        }
    }

    println!("{}", cost);
}
