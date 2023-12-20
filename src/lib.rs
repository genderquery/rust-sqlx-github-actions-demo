pub fn message() -> &'static str {
    "Hello, world!"
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_message() {
        assert_eq!(message(), "Hello, world!")
    }
}
