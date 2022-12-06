#[cfg(test)]
mod tests {
    use acme_minis::*;

    #[test]
    pub fn test_agency_default() {
        assert_eq!(Agency::default(), Agency::Client)
    }
}
