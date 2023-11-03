// utils is the sibling module of vector, so need to use this syntax
use super::utils;

#[no_mangle]
pub fn add_v(v1: i32, v2: i32) -> i32 {
    utils::add(v1, v2)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_add_v() {
        assert_eq!(add_v(1, 1), 2);
    }
}
