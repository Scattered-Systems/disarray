/*
   Appellation: provider <module>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
use crate::BoxedTransport;

#[derive(Debug, Default)]
pub struct Provider {
    pub address: String,
    pub clients: Vec<BoxedTransport>,
}

impl Provider {
    pub fn new(address: String, clients: Vec<BoxedTransport>) -> Self {
        Self { address, clients }
    }
}

impl std::fmt::Display for Provider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.address)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_provider() {
        let a = Provider::default();
        let b = Provider::new(Default::default(), Default::default());
        assert_eq!(&a.address, &b.address)
    }
}
