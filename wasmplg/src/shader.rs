#[no_mangle]
pub fn parse_shader() -> i32 {
    2077
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_shader() {
        assert_eq!(parse_shader(), 2077)
    }
}
