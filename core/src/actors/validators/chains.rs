/*
   Appellation: chains <validators>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
use super::determine_block_validity;
use crate::blocks::Block;

pub fn determine_chain_validity(chain: &[Block]) -> bool {
    for i in 0..chain.len() {
        if i == 0 {
            continue;
        }
        let first = chain.get(i - 1).expect("has to exist");
        let second = chain.get(i).expect("has to exist");
        if !determine_block_validity(second, first) {
            return false;
        }
    }
    true
}
