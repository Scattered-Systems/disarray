/*
    Appellation: fly <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
*/
use crate::{blocks::BlockHeader, Blockchain, ChainWrapper, ChainWrapperExt, CoreChainSpec};
use decanter::prelude::{Hash, Hashable};
use serde::{Deserialize, Serialize};

// FlyClientProposal is a proposal sent from the prover,
// it contains current chain depth and last block header.
#[derive(Clone, Debug, Deserialize, Eq, Hash, PartialEq, Serialize)]
pub struct Proposal {
    pub depth: usize,
    pub header: BlockHeader,
}

impl Proposal {
    pub fn new(depth: usize, header: BlockHeader) -> Self {
        Self { depth, header }
    }
}

impl Default for Proposal {
    fn default() -> Self {
        Self::new(0, Default::default())
    }
}

impl std::fmt::Display for Proposal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

impl std::convert::From<&Blockchain> for Proposal {
    fn from(chain: &Blockchain) -> Self {
        let depth = chain.depth() as usize;
        let header = chain.find_one_block(&chain.tip()).unwrap().header;
        Self::new(depth, header)
    }
}

impl std::convert::From<&Proposal> for Proposal {
    fn from(data: &Proposal) -> Self {
        data.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_proposal_defaults() {
        let a = Proposal::default();
        let b = Proposal::from(&a);
        assert_eq!(a, b);
    }
}
