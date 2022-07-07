/*
   Appellation: wallet
   Context:
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Wallet<Addr = String, Data = String> {
    pub address: Addr,
    pub data: Vec<Data>,
}

impl<Addr, Data> Wallet<Addr, Data> {
    pub fn constructor(address: Addr, data: Vec<Data>) -> Self {
        Self { address, data }
    }
    pub fn from(address: Addr) -> Self {
        Self::constructor(address, Vec::new())
    }
}

impl std::fmt::Display for Wallet {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Wallet(\naddress={:#?}, \ndata={:#?})", self.address, self.data)
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
