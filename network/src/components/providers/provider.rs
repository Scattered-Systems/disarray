/*
   Appellation: provider <module>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
use crate::BoxedTransport;

#[derive(Debug)]
pub struct Provider {
    pub address: String,
    pub clients: Vec<BoxedTransport>,
}

impl Provider {
    pub fn constructor(address: String, clients: Vec<BoxedTransport>) -> Self {
        Self { address, clients }
    }
}

impl std::fmt::Display for Provider {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Provider(\naddress={:#?})", self.address)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test() {
        let f = |x: usize| x.pow(x.try_into().unwrap());
        assert_eq!(f(2), 4)
    }
}
