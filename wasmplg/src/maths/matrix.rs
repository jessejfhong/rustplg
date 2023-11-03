// utils is the sibling module of matrix, so need to use this syntax
use super::utils;

#[no_mangle]
pub fn add_m(m1: i32, m2: i32) -> i32 {
    utils::add(m1, m2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_m() {
        assert_eq!(add_m(1, 1), 2);
    }
}
