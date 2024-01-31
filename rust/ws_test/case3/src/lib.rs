use common;

pub fn case3(left: usize, right: usize) -> usize {
    common::print_module_name();
    let foo = common::num::integer::lcm(left, right);
    right - left + foo
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn it_works() {
        let result = case3(2, 2);
        assert_eq!(result, 0);
    }
}
