#![feature(lazy_cell)]

use common::*;

pub fn case1(left: usize, right: usize) -> usize {
    print_module_name();
    foo!();
    let re = re!(r"test");
    println!("{}", re.find("needletest").unwrap().as_str());
    left + right
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case1_works() {
        let result = case1(2, 2);
        assert_eq!(result, 4);
    }
}
