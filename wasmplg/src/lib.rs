// to make these modules accessable within this file and ouside of the crate
pub mod maths;
pub mod parser;
pub mod shader;

#[no_mangle]
pub fn addu(left: usize, right: usize) -> usize {
    left + right
}

#[no_mangle]
pub fn factorial(n: u32) -> u32 {
    if n > 1 {
        n * factorial(n - 1)
    } else {
        1
    }
}

#[no_mangle]
pub fn multiply(n1: f32, n2: f32) -> f32 {
    n1 * n2
}

#[no_mangle]
pub fn max(n1: f64, n2: f64) -> f64 {
    if n1 > n2 {
        n1
    } else {
        n2
    }
}

#[no_mangle]
pub fn is_true(b: bool) -> bool {
    !b
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_addu() {
        assert_eq!(addu(2, 2), 4)
    }

    #[test]
    fn test_factorial() {
        assert_eq!(factorial(4), 24)
    }

    #[test]
    fn test_add_v() {
        assert_eq!(maths::vector::add_v(1, 1), 2)
    }
}
