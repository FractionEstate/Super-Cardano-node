#[cfg(test)]
mod validation_tests {
    use super::*;

    #[test]
    fn test_valid_configuration() {
        let config = default_configuration();
        assert!(validate(&config).is_ok());
    }

    #[test]
    fn test_invalid_port() {
        let mut config = default_configuration();
        config.network.port = 0;
        assert!(validate(&config).is_err());
    }
}
