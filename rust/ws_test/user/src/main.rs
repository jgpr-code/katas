#![feature(lazy_cell)]

use case1::case1;
use case2::case2;
use case3::case3;
use common::*;

fn main() {
    println!(
        "Hello, world {} {} {}!",
        case1(1, 2),
        case2(3, 4),
        case3(6, 6)
    );
    let re = re!(r"foo");
    println!("{}", re.find("barfoos").unwrap().as_str());
}
