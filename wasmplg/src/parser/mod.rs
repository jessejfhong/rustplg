#[no_mangle]
pub fn parse_int(_str: String) -> i32 {
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_int() {
        assert_eq!(parse_int(String::from("hh")), 1)
    }
}
