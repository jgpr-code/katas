use common::*;

pub fn case2(left: usize, right: usize) -> usize {
    print_module_name();
    let foo = num::integer::lcm(left, right);
    left * right + foo
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn case2_works() {
        let result = case2(2, 2);
        assert_eq!(result, 4);
    }
}
