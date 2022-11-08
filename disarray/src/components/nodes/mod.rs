/*
   Appellation: nodes <module>
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
pub use self::{attr::*, node::*, utils::*};

pub(crate) mod attr;
pub(crate) mod node;

pub(crate) mod utils {}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_node() {
        let a = Node::default();
        let b = a.clone();
        assert_eq!(&a.address, &b.address);
    }
}
