pub fn sayhello2(name: &str) -> String {
    format!("Hello, {}!", name)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sayhello2() {
        assert_eq!("Hello, Scrat!", sayhello2("Scrat"));
    }
}
