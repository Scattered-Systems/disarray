/*
    Appellation: types <module>
    Contrib: FL03 <jo3mccain@icloud.com>
    Description: ... summary ...
*/

pub type ChainError = Box<dyn std::error::Error + Send + Sync>;
pub type ChainResult<T = (), E = ChainError> = Result<T, E>;
