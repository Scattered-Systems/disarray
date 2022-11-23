/*
    Appellation: fly <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description:
*/
use crate::{
    blockchains::{Blockchain, ChainWrapperExt, CoreChainSpec},
    blocks::BlockHeader,
};
use scsys::{prelude::*, Hashable};
use serde::{Deserialize, Serialize};

// FlyClientProposal is a proposal sent from the prover,
// it contains current chain depth and last block header.
#[derive(Clone, Debug, Deserialize, Eq, Hash, Hashable, PartialEq, Serialize)]
pub struct Proposal {
    pub depth: usize,
    pub header: BlockHeader,
}

impl Proposal {
    pub fn new(blockchain: &Blockchain) -> Self {
        let depth = blockchain.position.depth as usize;
        let header = blockchain.find_one_block(&blockchain.tip()).unwrap().header;
        Self { depth, header }
    }
}

impl Default for Proposal {
    fn default() -> Self {
        Self::new(&Default::default())
    }
}

impl std::fmt::Display for Proposal {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_proposal_defaults() {
        let a = Proposal::default();
        assert!(true);
    }
}
