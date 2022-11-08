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
    fn test_basic_node() {
        let a = 0;
        let b = 0;
        assert_eq!(&a, &b);
    }
}
