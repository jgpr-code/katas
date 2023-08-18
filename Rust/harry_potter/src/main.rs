use std::env;

fn compute_price(books: &[u32]) -> f64 {
    let mut books_vec = books.to_vec();
    let mut count: u32 = books_vec.iter().sum();
    let mut price: u128 = 0;
    while count > 0 {
        // greedy approach
        let mut packaged: u32 = 0;
        for i in 0..5 {
            if books_vec[i] > 0 {
                books_vec[i] -= 1;
                packaged += 1;
            }
        }

        // each costs 8
        // 2 different 5% discount
        // 3 different 10% discount
        // 4 different 20% discount
        // 5 different 25% discount
        let discount = match packaged {
            2 => 95,
            3 => 90,
            4 => 80,
            5 => 75,
            _ => 100,
        };
        price += packaged as u128 * 8 * discount;
        count -= packaged;
    }
    price as f64 / 100.0
}

fn main() {
    println!("Hello, world!");
    let mut args: Vec<u32> = env::args()
        .skip(1)
        .map(|s| s.parse::<u32>().unwrap())
        .collect();
    println!("{:?}", args);
    let price = compute_price(&mut args);
    println!("{}", price);
}
