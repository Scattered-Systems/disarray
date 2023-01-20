/*
    Appellation: clients <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use decanter::prelude::{Hash, Hashable};
use serde::{Deserialize, Serialize};
use std::convert::From;

// FlyClientQuery is the query sent from verifier to prover,
// it contains the chain depth of a proposal and a sample of
// blocks for proof. Note sample points are < query_depth - 1.
#[derive(
    Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, Serialize,
)]
pub struct Query {
    pub depth: usize,
    pub sample: Vec<usize>,
}

impl Query {
    pub fn new(depth: usize, sample: Vec<usize>) -> Self {
        Self { depth, sample }
    }
}

impl std::fmt::Display for Query {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string(&self).unwrap())
    }
}

impl From<(usize, Vec<usize>)> for Query {
    fn from(data: (usize, Vec<usize>)) -> Self {
        Self::new(data.0, data.1)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_query_defaults() {
        let a = Query::default();
        let b = Query::from((Default::default(), Default::default()));
        assert_eq!(&a, &b);
    }
}
