/*
   Appellation: common
   Context:
   Creator: FL03 <jo3mccain@icloud.com>
   Description:
       ... Summary ...
*/
pub use constants::*;
pub use types::*;
pub use variants::*;

mod constants {
    pub const DIFFICULTY_PREFIX: &str = "00";
}

mod types {}

mod variants {
    pub enum Dates {
        Standard(i64),
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
