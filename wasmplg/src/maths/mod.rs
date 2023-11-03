pub mod matrix;
pub mod vector;

// private modules, accessable within maths module only.
mod utils;

#[no_mangle]
pub fn sub(n1: i32, n2: i32) -> i32 {
    n1 - n2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sub() {
        assert_eq!(sub(3, 2), 1)
    }

    #[test]
    fn test_matrix() {
        assert_eq!(matrix::add_m(1, 2), 3)
    }
}
