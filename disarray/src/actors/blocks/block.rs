/*
   Appellation: block
   Context:
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/

#[derive(Clone, Debug, Hash, PartialEq, serde::Deserialize, serde::Serialize)]
pub struct Block<Data = String, Id = u8, Hash = Vec<u8>, Nonce = u8, Ts = i64> {
    pub id: Id,
    pub hash: Hash,
    pub nonce: Nonce,
    pub previous: Hash,
    pub timestamp: Ts,
    pub data: Vec<Data>,
}

impl<Data, Id, Hash, Nonce, Ts> Block<Data, Id, Hash, Nonce, Ts> {
    pub fn constructor(
        id: Id,
        hash: Hash,
        nonce: Nonce,
        previous: Hash,
        timestamp: Ts,
        data: Vec<Data>,
    ) -> Self {
        Self {
            id,
            hash,
            nonce,
            previous,
            timestamp,
            data,
        }
    }
}

impl std::fmt::Display for Block {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "Block(\nid={:#?},\nhash={:#?},\nnonce={:#?},\nprevious={:#?},\ntimestamp={:#?},\ndata={:#?})",
            self.id, self.hash, self.nonce, self.previous, self.timestamp, self.data
        )
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
