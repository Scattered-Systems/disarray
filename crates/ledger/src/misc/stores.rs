/*
    Appellation: stores <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... Summary ...
*/
use ckb_merkle_mountain_range::MMRStore;
use decanter::prelude::{Hash, Hashable, H256};
use scsys::prelude::SerdeDisplay;
use serde::{Deserialize, Serialize};

///
#[derive(Clone, Debug, Default, Deserialize, Eq, Hash, Ord, PartialEq, PartialOrd, SerdeDisplay, Serialize)]
pub struct BlockStore<T: Clone = H256>(pub Vec<T>);

impl<T: Clone> MMRStore<T> for BlockStore<T> {
    fn get_elem(&self, pos: u64) -> ckb_merkle_mountain_range::Result<Option<T>> {
        Ok(Some(self.0[pos as usize].clone()))
    }

    fn append(&mut self, pos: u64, elems: Vec<T>) -> ckb_merkle_mountain_range::Result<()> {
        let mut data = elems;
        let mut tmp = Vec::new();
        for i in (std::ops::Range {
            start: 0,
            end: self.0.len(),
        }) {
            if i == pos as usize {
                tmp.append(&mut data);
            } else {
                tmp.push(self.0[i].clone())
            }
        }

        Ok(())
    }
}
