/*
   Appellation: node
   Context:
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Node<Addr = String> {
    pub address: Addr,
}

impl<Addr> Node<Addr> {
    pub fn constructor(address: Addr) -> Self {
        Self { address }
    }
    pub fn from(address: Addr) -> Self {
        Self::constructor(address)
    }
}

impl std::fmt::Display for Node {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Node(address={})", self.address)
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
