/*
   Appellation: contents <blocks>
   Contributors: FL03 <jo3mccain@icloud.com>
   Description: ... Summary ...
*/
pub use self::{content::BlockContent, utils::*};

pub(crate) mod content;

pub(crate) mod utils {
    use super::BlockContent;
    use crate::transactions::generate_random_signed_transaction;
    use scsys::prelude::generate_random_hash;

    pub fn generate_random_block_content() -> BlockContent {
        BlockContent::new(
            vec![generate_random_signed_transaction()],
            vec![generate_random_hash()],
        )
    }
}
