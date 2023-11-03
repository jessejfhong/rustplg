#[no_mangle]
pub fn add(n1: i32, n2: i32) -> i32 {
    n1 + n2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_multiply2() {
        assert_eq!(add(2, 3), 5);
    }
}
